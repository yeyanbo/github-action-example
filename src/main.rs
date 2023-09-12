use axum::{Extension, Router};
use axum::response::Html;
use axum::routing::get;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // let args = Args::from_args_safe()?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let routes = Router::new()
        .route("/", get(welcome));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {} ...", &addr);

    axum::Server::bind(&addr)
        //使用这种调用方式替代nto_make_service，可以获取客户端连接信息
        .serve(routes.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn welcome() -> Html<&'static str> {
    Html("<h1>Hello!</h1><p>Welcome to FHIR Connectathon Platform!</p>")
}
