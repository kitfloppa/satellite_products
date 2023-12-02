pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod routes;
pub(crate) mod service;

#[cfg(feature = "diesel")]
use diesel_async::pooled_connection::deadpool::Pool;
#[cfg(feature = "diesel")]
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

use dotenv::dotenv;
use model::satellite::Satellite;
use model::satellite_data::SatelliteData;
use repository::InMemoryRepository;
use service::oceancolor::{OceanColorJobSettings, OceanColorServiceDefault};
use service::satellite::SatelliteServiceMock;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio_cron_scheduler::JobScheduler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    // fetch environment variables
    #[cfg(feature = "diesel")]
    {
        let database_connection_url = std::env::var("DATABASE_URL")?;
    }
    let server_ip = std::env::var("SERVER_IP")?;

    let oceancolor_authorization = std::env::var("OCEANCOLOR_AUTHORIZATION")?;
    let oceancolor_job_timestep = std::env::var("OCEANCOLOR_JOB_TIMESTEP")?.parse::<u64>()?;
    let oceancolor_job_notfound = std::env::var("OCEANCOLOR_JOB_NOTFOUND")?.parse::<i64>()?;

    // config connection with database
    #[cfg(feature = "diesel")]
    {
        let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
            database_connection_url,
        );
        let pool = tokio::sync::Mutex::new(Pool::builder(config).build()?);
    }

    // construct repositories
    let tles = std::fs::read_to_string("celestrak.txt")
        .expect("failed to load celestrak data")
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|slice| Satellite::new(slice[0], slice[1], slice[2]))
        .collect::<Vec<_>>();

    let satellite_repository = Arc::new(tokio::sync::RwLock::new(
        InMemoryRepository::<Satellite>::from(tles),
    ));

    let satellite_data_repository = Arc::new(tokio::sync::RwLock::new(InMemoryRepository::<
        SatelliteData,
    >::new()));

    // construct services
    let satellite_serivce = SatelliteServiceMock::new(satellite_repository.clone());

    let oceancolor_service = Arc::new(OceanColorServiceDefault::new(
        oceancolor_authorization,
        satellite_data_repository.clone(),
    ));

    // setup job scheduler
    let job_scheduler = JobScheduler::new().await?;

    let oceancolor_job = oceancolor_service.create_job(OceanColorJobSettings {
        time_step: std::time::Duration::from_secs(oceancolor_job_timestep),
        not_found_duration: chrono::Duration::seconds(oceancolor_job_notfound),
    })?;
    job_scheduler.add(oceancolor_job).await?;

    job_scheduler.start().await?;

    // startup application
    let app_context = routes::AppContext {
        #[cfg(feature = "diesel")]
        pool,
        satellite_service: Arc::new(satellite_serivce),
        oceancolor_service,
        satellite_repository,
        satellite_data_repository,
        job_scheduler,
    };
    let app = routes::router(app_context);

    let addr = server_ip.parse::<SocketAddr>()?;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
