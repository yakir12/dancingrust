use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;
// use build_html::{Table, Html};
use build_html::{HtmlPage, HtmlContainer, Html};

async fn hello_world() -> String {
    // let source_table = [
    //     [1, 2, 3],
    //     [4, 5, 6],
    //     [7, 8, 9]
    // ];

    // let html_table = Table::from(source_table)
    //     .with_header_row(['A', 'B', 'C'])
    //     .to_html_string();

    // return html_table

    let page: String = HtmlPage::new()
    .with_title("My Page")
    .with_header(1, "Header Text")
    .to_html_string();

    return page
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let router = Router::new().route("/", get(hello_world));
    let addr = SocketAddr::from(([0,0,0,0], 8080));
    let tcp = TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcp, router).await.unwrap();
}
