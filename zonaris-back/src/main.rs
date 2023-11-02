pub(crate) mod model {
    pub(crate) mod satellite;
}

pub(crate) mod service {
    pub(crate) mod oceancolor;
    pub(crate) mod satellite;
}

pub(crate) mod routes;

use chrono::{prelude::*, Duration};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenv::dotenv;
use model::satellite::Satellite;
use service::oceancolor::{OceanColorService, OceanColorServiceDefault};
use service::satellite::SatelliteServiceMock;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let database_connection_url = std::env::var("DATABASE_URL")?;
    let server_ip = std::env::var("SERVER_IP")?;
    let oceancolor_authorization = std::env::var("OCEANCOLOR_AUTHORIZATION")?;

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

    let oceancolor_service = OceanColorServiceDefault::new(oceancolor_authorization);

    // let items = oceancolor_service
    //     .search(
    //         Utc::now()
    //             .checked_sub_signed(Duration::hours(8))
    //             .unwrap()
    //             .naive_utc(),
    //         Utc::now().naive_utc(),
    //     )
    //     .await?;

    // println!("Num items: {}", items.len());

    // let images =
    //     futures::future::join_all(items.into_iter().map(|item| oceancolor_service.get(item))).await;

    // let mut i = 0;
    // for image in images {
    //     image?.save(format!("images/{}.png", i))?;
    //     i += 1;
    // }

    let app_context = routes::AppContext {
        pool,
        satellite_service: Arc::new(satellite_serivce),
        oceancolor_service: Arc::new(oceancolor_service),
    };
    let app = routes::router(app_context);

    let addr = server_ip.parse::<SocketAddr>().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
