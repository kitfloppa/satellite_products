pub mod controller;
pub mod dto;
pub mod persistence;
pub mod routes;
pub mod service;
pub mod utils;

use dotenv::dotenv;
use persistence::create_inmemory_repository;
use persistence::model::instrument::Instrument;
use persistence::model::instrument_data::InstrumentData;
use persistence::model::oceancolor::OceanColorMapping;
use persistence::model::satellite::Satellite;
use persistence::model::satellite_instrument::SatelliteInstrument;
use persistence::repository::Repository;
use service::instrument_data::InstrumentDataServiceDefault;
use service::oceancolor::{OceanColorJobSettings, OceanColorServiceDefault};
use service::satellite::SatelliteServiceMock;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio_cron_scheduler::JobScheduler;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[cfg(feature = "diesel")]
use diesel_async::pooled_connection::deadpool::Pool;
#[cfg(feature = "diesel")]
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

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
    let satellite_repository = create_inmemory_repository::<Satellite>();
    let instrument_repository = create_inmemory_repository::<Instrument>();
    let satellite_instrument_repository = create_inmemory_repository::<SatelliteInstrument>();
    let instrument_data_repository = create_inmemory_repository::<InstrumentData>();
    let oceancolor_mapping_repository = create_inmemory_repository::<OceanColorMapping>();

    // add test data
    {
        let mut lock = satellite_repository.write().await;
        lock.add(Satellite::new(
            "TERRA",
            "1 25994U 99068A   23336.53168247  .00001150  00000+0  25187-3 0  9996",
            "2 25994  98.0761  40.6034 0000638 235.8264 235.6779 14.59469347274213",
        ))
        .await;
    }

    {
        let mut lock = instrument_repository.write().await;
        lock.add(Instrument::new("MODIS")).await;
    }

    {
        let mut lock = satellite_instrument_repository.write().await;
        lock.add(SatelliteInstrument::new(0, 0)).await; // relax relax just for test
    }

    {
        let mut lock = oceancolor_mapping_repository.write().await;
        lock.add(OceanColorMapping::new(0, 8, 1102)).await; // relax relax just for test
    }

    // construct services
    let satellite_service = Arc::new(SatelliteServiceMock::new(satellite_repository.clone()));

    let instrument_data_service = Arc::new(InstrumentDataServiceDefault::new(
        satellite_instrument_repository.clone(),
        instrument_data_repository.clone(),
    ));

    let oceancolor_service = Arc::new(OceanColorServiceDefault::new(
        oceancolor_authorization,
        oceancolor_mapping_repository.clone(),
        instrument_data_service.clone(),
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
    let app_context = Arc::new(routes::AppContext {
        #[cfg(feature = "diesel")]
        pool,
        satellite_service,
        oceancolor_service,
        satellite_repository,
        instrument_repository,
        satellite_instrument_repository,
        instrument_data_service,
        instrument_data_repository,
        oceancolor_mapping_repository,
        job_scheduler,
    });

    let app = routes::create_router(app_context)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let addr = server_ip.parse::<SocketAddr>()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controller::instrument_data::get_by_satellite_id,
        // crate::controller::instrument_data::get_asset,
        crate::controller::satellite::get_all,
    ),
    components(
        schemas(
            crate::dto::instrument_data::InstrumentDataResponse,
            crate::dto::satellite::SatelliteResponse
        )
    )
)]
struct ApiDoc;
