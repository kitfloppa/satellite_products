pub(crate) mod model {
    pub(crate) mod satellite;
}

pub(crate) mod service {
    pub(crate) mod satellite;
}

pub(crate) mod routes;

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenv::dotenv;
use model::satellite::Satellite;
use service::satellite::SatelliteServiceMock;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_connection_url = std::env::var("DATABASE_URL").unwrap();
    let server_ip = std::env::var("SERVER_IP").unwrap();

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
        database_connection_url,
    );
    let pool = Pool::builder(config).build().unwrap();

    let tles = std::fs::read_to_string("celestrak.txt")
        .expect("failed to load celestrak data")
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|slice| Satellite::new(slice[0], slice[1], slice[2]))
        .collect::<Vec<_>>();

    let satellite_serivce = SatelliteServiceMock::new(tles);

    let app_context = routes::AppContext {
        pool,
        satellite_service: Arc::new(satellite_serivce),
    };
    let app = routes::router(app_context);

    let addr = server_ip.parse::<SocketAddr>().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
