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

    let satellite_serivce = SatelliteServiceMock::new(vec![
        Satellite::new("SPACE STATION", -31.34, -17.80),
        Satellite::new("SES 1", -0.02, -100.99),
        Satellite::new("NOAA 19", -8.29, -169.15),
    ]);

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
