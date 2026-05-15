use axum::{
    body::Body,
    extract::MatchedPath,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use opentelemetry::{
    global,
    metrics::{Counter, Histogram},
    trace::TracerProvider as _,
    KeyValue,
};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, MetricExporter, SpanExporter};
use opentelemetry_sdk::{
    logs::SdkLoggerProvider,
    metrics::SdkMeterProvider,
    propagation::TraceContextPropagator,
    trace::SdkTracerProvider,
    Resource,
};
use std::{
    error::Error,
    sync::OnceLock,
    time::{Duration, Instant},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

type BoxError = Box<dyn Error + Send + Sync + 'static>;

#[derive(Clone)]
pub struct Telemetry {
    tracer_provider: SdkTracerProvider,
    meter_provider: SdkMeterProvider,
    logger_provider: SdkLoggerProvider,
}

pub fn init(service_name: &str) -> Result<Telemetry, BoxError> {
    let resource = Resource::builder()
        .with_service_name(service_name.to_owned())
        .build();

    global::set_text_map_propagator(TraceContextPropagator::new());

    let span_exporter = SpanExporter::builder().with_http().build().map_err(to_boxed)?;
    let tracer_provider = SdkTracerProvider::builder()
        .with_resource(resource.clone())
        .with_batch_exporter(span_exporter)
        .build();
    let tracer = tracer_provider.tracer(service_name.to_owned());
    global::set_tracer_provider(tracer_provider.clone());

    let metric_exporter = MetricExporter::builder()
        .with_http()
        .build()
        .map_err(to_boxed)?;
    let meter_provider = SdkMeterProvider::builder()
        .with_resource(resource.clone())
        .with_periodic_exporter(metric_exporter)
        .build();
    global::set_meter_provider(meter_provider.clone());

    let log_exporter = LogExporter::builder().with_http().build().map_err(to_boxed)?;
    let logger_provider = SdkLoggerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(log_exporter)
        .build();

    tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(
            OpenTelemetryTracingBridge::new(&logger_provider)
                .with_filter(bridge_filter(service_name)),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_names(true)
                .with_filter(default_filter(service_name)),
        )
        .init();

    Ok(Telemetry {
        tracer_provider,
        meter_provider,
        logger_provider,
    })
}

impl Telemetry {
    pub fn shutdown(self) -> Result<(), BoxError> {
        self.meter_provider.shutdown().map_err(to_boxed)?;
        self.logger_provider.shutdown().map_err(to_boxed)?;
        self.tracer_provider.shutdown().map_err(to_boxed)?;
        Ok(())
    }
}

pub async fn http_telemetry_middleware(req: Request<Body>, next: Next) -> Response {
    let method = req.method().to_string();
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map(MatchedPath::as_str)
        .unwrap_or_else(|| req.uri().path())
        .to_owned();

    let started_at = Instant::now();
    let response = next.run(req).await;
    let status = response.status();
    let attributes = http_attributes(&method, &route, status);

    http_metrics().request_total.add(1, &attributes);
    http_metrics()
        .request_duration_seconds
        .record(started_at.elapsed().as_secs_f64(), &attributes);

    if status.is_server_error() {
        http_metrics().server_errors_total.add(1, &attributes);
    }

    response
}

pub fn record_image_stitch_success(
    operation: &'static str,
    image_count: usize,
    duration: Duration,
    output_bytes: usize,
    output_width: u32,
    output_height: u32,
) {
    let attributes = image_stitch_attributes(operation, "success");

    image_stitch_metrics()
        .jobs_total
        .add(1, &attributes);
    image_stitch_metrics()
        .input_images
        .record(image_count as u64, &attributes);
    image_stitch_metrics()
        .processing_duration_seconds
        .record(duration.as_secs_f64(), &attributes);
    image_stitch_metrics()
        .output_bytes
        .record(output_bytes as u64, &attributes);
    image_stitch_metrics()
        .output_width_pixels
        .record(output_width as u64, &attributes);
    image_stitch_metrics()
        .output_height_pixels
        .record(output_height as u64, &attributes);
}

pub fn record_image_stitch_failure(
    operation: &'static str,
    image_count: usize,
    duration: Duration,
) {
    let attributes = image_stitch_attributes(operation, "error");

    image_stitch_metrics()
        .jobs_total
        .add(1, &attributes);
    image_stitch_metrics()
        .failures_total
        .add(1, &attributes);
    image_stitch_metrics()
        .input_images
        .record(image_count as u64, &attributes);
    image_stitch_metrics()
        .processing_duration_seconds
        .record(duration.as_secs_f64(), &attributes);
}

fn default_filter(service_name: &str) -> EnvFilter {
    EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("{service_name}=info,tower_http=info")))
}

fn bridge_filter(service_name: &str) -> EnvFilter {
    [
        "opentelemetry=off",
        "hyper=off",
        "reqwest=off",
        "h2=off",
    ]
    .into_iter()
    .fold(default_filter(service_name), |filter, directive| {
        filter.add_directive(directive.parse().expect("valid filter directive"))
    })
}

fn http_attributes(method: &str, route: &str, status: StatusCode) -> [KeyValue; 3] {
    [
        KeyValue::new("http.request.method", method.to_owned()),
        KeyValue::new("http.route", route.to_owned()),
        KeyValue::new(
            "http.response.status_code",
            i64::from(status.as_u16()),
        ),
    ]
}

fn image_stitch_attributes(operation: &'static str, outcome: &'static str) -> [KeyValue; 2] {
    [
        KeyValue::new("image_stitch.operation", operation),
        KeyValue::new("image_stitch.outcome", outcome),
    ]
}

fn http_metrics() -> &'static HttpMetrics {
    static HTTP_METRICS: OnceLock<HttpMetrics> = OnceLock::new();
    HTTP_METRICS.get_or_init(HttpMetrics::new)
}

fn image_stitch_metrics() -> &'static ImageStitchMetrics {
    static IMAGE_STITCH_METRICS: OnceLock<ImageStitchMetrics> = OnceLock::new();
    IMAGE_STITCH_METRICS.get_or_init(ImageStitchMetrics::new)
}

fn to_boxed<E>(error: E) -> BoxError
where
    E: Error + Send + Sync + 'static,
{
    Box::new(error)
}

struct HttpMetrics {
    request_total: Counter<u64>,
    server_errors_total: Counter<u64>,
    request_duration_seconds: Histogram<f64>,
}

impl HttpMetrics {
    fn new() -> Self {
        let meter = global::meter("rust-http-service");

        Self {
            request_total: meter.u64_counter("http.server.request.count").build(),
            server_errors_total: meter.u64_counter("http.server.error.count").build(),
            request_duration_seconds: meter.f64_histogram("http.server.request.duration").build(),
        }
    }
}

struct ImageStitchMetrics {
    jobs_total: Counter<u64>,
    failures_total: Counter<u64>,
    input_images: Histogram<u64>,
    processing_duration_seconds: Histogram<f64>,
    output_bytes: Histogram<u64>,
    output_width_pixels: Histogram<u64>,
    output_height_pixels: Histogram<u64>,
}

impl ImageStitchMetrics {
    fn new() -> Self {
        let meter = global::meter("image-stitch");

        Self {
            jobs_total: meter.u64_counter("image_stitch.processing.jobs").build(),
            failures_total: meter.u64_counter("image_stitch.processing.failures").build(),
            input_images: meter.u64_histogram("image_stitch.processing.input_images").build(),
            processing_duration_seconds: meter
                .f64_histogram("image_stitch.processing.duration")
                .build(),
            output_bytes: meter.u64_histogram("image_stitch.processing.output_bytes").build(),
            output_width_pixels: meter
                .u64_histogram("image_stitch.processing.output_width")
                .build(),
            output_height_pixels: meter
                .u64_histogram("image_stitch.processing.output_height")
                .build(),
        }
    }
}
