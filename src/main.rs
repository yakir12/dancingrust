use axum::{Router, routing::get, response::Html as AxumHtml};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use build_html::{Table, Html};

async fn hello_world() -> AxumHtml<String> {
    let source_table = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];

    let html_table = Table::from(source_table)
        .with_header_row(['A', 'B', 'C'])
        .to_html_string();
    
    // Create a complete HTML document
    let full_html = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>3x3 Table</title>
            <style>
                table {{ border-collapse: collapse; margin: 25px 0; }}
                table td, table th {{ border: 1px solid #ddd; padding: 8px; }}
            </style>
        </head>
        <body>
            <h1>My 3x3 Table</h1>
            {}
        </body>
        </html>
    "#, html_table);
    
    // Use Axum's Html type to set the correct Content-Type header
    AxumHtml(full_html)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let router = Router::new().route("/", get(hello_world));
    let addr = SocketAddr::from(([0,0,0,0], 8080));
    let tcp = TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcp, router).await.unwrap();
}