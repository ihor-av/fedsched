use fedsched::setup_scheduler;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let router = setup_scheduler().await;

    println!("Router running on http://localhost:8000/graphql");
    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), router)
        .await
        .unwrap();
}
