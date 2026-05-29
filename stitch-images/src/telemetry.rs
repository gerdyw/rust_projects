use std::time::Duration;

use opentelemetry::{
    KeyValue, global,
    metrics::{Histogram, MeterProvider as _},
    trace::TracerProvider as _,
};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, MetricExporter, Protocol, SpanExporter, WithExportConfig};
use opentelemetry_sdk::{
    Resource,
    logs::SdkLoggerProvider,
    metrics::SdkMeterProvider,
    trace::SdkTracerProvider,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type TelemetryResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[derive(Clone)]
pub struct TelemetryMetrics {
    submission_duration_ms: Histogram<f64>,
    processing_duration_ms: Histogram<f64>,
}

impl TelemetryMetrics {
    pub fn record_submission_duration(&self, duration: Duration, outcome: &'static str) {
        self.submission_duration_ms.record(
            duration.as_secs_f64() * 1000.0,
            &[KeyValue::new("outcome", outcome)],
        );
    }

    pub fn record_processing_duration(
        &self,
        duration: Duration,
        outcome: &'static str,
        image_count: u32,
    ) {
        self.processing_duration_ms.record(
            duration.as_secs_f64() * 1000.0,
            &[
                KeyValue::new("outcome", outcome),
                KeyValue::new("image_count", i64::from(image_count)),
            ],
        );
    }
}

pub struct Telemetry {
    tracer_provider: SdkTracerProvider,
    meter_provider: SdkMeterProvider,
    logger_provider: SdkLoggerProvider,
    metrics: TelemetryMetrics,
}

impl Telemetry {
    pub fn init(service_name: &'static str) -> TelemetryResult<Self> {
        let resource = Resource::builder().with_service_name(service_name).build();

        let tracer_provider = SdkTracerProvider::builder()
            .with_resource(resource.clone())
            .with_batch_exporter(
                SpanExporter::builder()
                    .with_http()
                    .with_protocol(Protocol::HttpBinary)
                    .build()?,
            )
            .build();

        let meter_provider = SdkMeterProvider::builder()
            .with_resource(resource.clone())
            .with_periodic_exporter(
                MetricExporter::builder()
                    .with_http()
                    .with_protocol(Protocol::HttpBinary)
                    .build()?,
            )
            .build();

        let logger_provider = SdkLoggerProvider::builder()
            .with_resource(resource)
            .with_batch_exporter(
                LogExporter::builder()
                    .with_http()
                    .with_protocol(Protocol::HttpBinary)
                    .build()?,
            )
            .build();

        global::set_tracer_provider(tracer_provider.clone());
        global::set_meter_provider(meter_provider.clone());

        let tracer = tracer_provider.tracer(service_name);
        let log_layer = OpenTelemetryTracingBridge::new(&logger_provider);

        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "stitch_images=info,rocket=info".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .with(log_layer)
            .try_init()?;

        let meter = meter_provider.meter(service_name);
        let metrics = TelemetryMetrics {
            submission_duration_ms: meter
                .f64_histogram("stitch_images.submission.duration")
                .with_unit("ms")
                .with_description("Time spent accepting and storing submitted images.")
                .build(),
            processing_duration_ms: meter
                .f64_histogram("stitch_images.processing.duration")
                .with_unit("ms")
                .with_description("Time spent stitching submitted images.")
                .build(),
        };

        Ok(Self {
            tracer_provider,
            meter_provider,
            logger_provider,
            metrics,
        })
    }

    pub fn metrics(&self) -> TelemetryMetrics {
        self.metrics.clone()
    }

    pub fn shutdown(&self) {
        if let Err(err) = self.logger_provider.shutdown() {
            tracing::warn!(error = %err, "failed to shutdown logger provider");
        }

        if let Err(err) = self.meter_provider.shutdown() {
            tracing::warn!(error = %err, "failed to shutdown meter provider");
        }

        if let Err(err) = self.tracer_provider.shutdown() {
            tracing::warn!(error = %err, "failed to shutdown tracer provider");
        }
    }
}
