use uuid::Uuid;

/// Waiting page with loading spinner and HTMX polling
pub fn waiting_page(job_id: Uuid) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Processing Images...</title>
    <script src="https://unpkg.com/htmx.org@1.9.10"></script>
    <style>
        body {{
            font-family: system-ui, -apple-system, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            margin: 0;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
        }}
        .container {{
            text-align: center;
            padding: 2rem;
        }}
        .spinner {{
            border: 4px solid rgba(255, 255, 255, 0.3);
            border-radius: 50%;
            border-top: 4px solid white;
            width: 60px;
            height: 60px;
            animation: spin 1s linear infinite;
            margin: 2rem auto;
        }}
        @keyframes spin {{
            0% {{ transform: rotate(0deg); }}
            100% {{ transform: rotate(360deg); }}
        }}
        h1 {{
            font-size: 2rem;
            margin-bottom: 0.5rem;
        }}
        #status {{
            margin-top: 1rem;
            font-size: 1.1rem;
            opacity: 0.9;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Processing Your Images</h1>
        <div class="spinner"></div>
        <div id="status" 
             hx-get="/status/{}" 
             hx-trigger="load, every 1s" 
             hx-swap="innerHTML">
            Initializing...
        </div>
    </div>
</body>
</html>"#,
        job_id
    )
}

/// Status fragments for HTMX polling
pub fn status_pending() -> String {
    "<span>⏳ Waiting to start...</span>".to_string()
}

pub fn status_processing(image_count: i32) -> String {
    format!("<span>⚙️ Processing {} images...</span>", image_count)
}

pub fn status_not_found() -> String {
    "<span>Job not found</span>".to_string()
}

pub fn status_error() -> String {
    "<span>Error checking status</span>".to_string()
}

/// Error page template
pub fn error_page(title: &str, message: &str, details: Option<&str>) -> String {
    let details_html = details
        .map(|d| format!(r#"<p style="font-size: 0.9rem; color: #666;">{}</p>"#, d))
        .unwrap_or_default();

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{
            font-family: system-ui, -apple-system, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            margin: 0;
            background: #f5f5f5;
        }}
        .error {{
            text-align: center;
            padding: 2rem;
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }}
        h1 {{ color: #e53e3e; }}
    </style>
</head>
<body>
    <div class="error">
        <h1>❌ {}</h1>
        <p>{}</p>
        {}
    </div>
</body>
</html>"#,
        title, title, message, details_html
    )
}

/// Job not found error page
pub fn job_not_found(job_id: Uuid) -> String {
    error_page(
        "Job Not Found",
        &format!("Job ID: {}", job_id),
        None,
    )
}

/// Processing failed page with error details
pub fn processing_failed(error_message: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Processing Failed</title>
    <style>
        body {{
            font-family: system-ui, -apple-system, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            margin: 0;
            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
            color: white;
        }}
        .container {{
            text-align: center;
            padding: 2rem;
            max-width: 600px;
        }}
        .error-icon {{
            font-size: 4rem;
            margin-bottom: 1rem;
        }}
        h1 {{
            font-size: 2rem;
            margin-bottom: 1rem;
        }}
        .error-details {{
            background: rgba(255, 255, 255, 0.1);
            padding: 1rem;
            border-radius: 8px;
            margin-top: 1rem;
            word-wrap: break-word;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="error-icon">❌</div>
        <h1>Image Processing Failed</h1>
        <p>We encountered an error while processing your images.</p>
        <div class="error-details">
            <strong>Error:</strong><br>
            {}
        </div>
    </div>
</body>
</html>"#,
        error_message
    )
}
