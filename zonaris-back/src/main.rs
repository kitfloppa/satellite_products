pub mod controller;
pub mod dto;
pub mod persistence;
pub mod routes;
pub mod service;
pub mod utils;

use anyhow::Result;
use dotenv::dotenv;
use itertools::Itertools;
use persistence::create_inmemory_repository;
use persistence::model::instrument::Instrument;
use persistence::model::instrument_data::InstrumentData;
use persistence::model::oceancolor::OceanColorMapping;
use persistence::model::satellite::Satellite;
use persistence::model::satellite_instrument::SatelliteInstrument;
use service::celestrak::CelestrakServiceDefault;
use service::instrument_data::InstrumentDataServiceDefault;
use service::oceancolor::{OceanColorJobSettings, OceanColorServiceDefault};
use service::satellite::SatelliteServiceMock;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio_cron_scheduler::JobScheduler;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[cfg(feature = "postgres")]
use tokio::sync::Mutex;

#[cfg(feature = "postgres")]
use persistence::postgres::{create_postgres_repository, migration::migrate};

async fn get_satellite_by_catnr(
    celestrak_service: &service::CelestrakService,
    catnr: u32,
) -> Result<Satellite> {
    return Ok(Satellite::from(
        celestrak_service
            .gp_query(service::celestrak::Query::CATNR(catnr))
            .await?
            .into_iter()
            .exactly_one()?,
    ));
}

async fn add_test_data(
    celestrak_service: service::CelestrakService,
    satellite_repository: persistence::Repository<Satellite>,
    instrument_repository: persistence::Repository<Instrument>,
    satellite_instrument_repository: persistence::Repository<SatelliteInstrument>,
    oceancolor_mapping_repository: persistence::Repository<OceanColorMapping>,
) -> Result<()> {
    let (terra, aqua, s3a) = {
        let terra = get_satellite_by_catnr(&celestrak_service, 25994).await?;
        let aqua = get_satellite_by_catnr(&celestrak_service, 27424).await?;
        let s3a = get_satellite_by_catnr(&celestrak_service, 41335).await?;

        let mut lock = satellite_repository.write().await;

        // TODO: delete unwrap
        (
            lock.add(terra).await?.unwrap(),
            lock.add(aqua).await?.unwrap(),
            lock.add(s3a).await?.unwrap(),
        )
    };

    let (modis, olci) = {
        let mut lock = instrument_repository.write().await;

        // TODO: delete unwrap
        (
            lock.add(Instrument::new("MODIS")).await?.unwrap(),
            lock.add(Instrument::new("OLCI")).await?.unwrap(),
        )
    };

    let (terra_modis, aqua_modis, s3a_olci) = {
        let mut lock = satellite_instrument_repository.write().await;

        // TODO: delete unwrap
        (
            lock.add(SatelliteInstrument::new(terra, modis))
                .await?
                .unwrap(),
            lock.add(SatelliteInstrument::new(aqua, modis))
                .await?
                .unwrap(),
            lock.add(SatelliteInstrument::new(s3a, olci))
                .await?
                .unwrap(),
        )
    };

    {
        let mut lock = oceancolor_mapping_repository.write().await;
        lock.add(OceanColorMapping::new(terra_modis, 8, 1102))
            .await?;
        lock.add(OceanColorMapping::new(aqua_modis, 7, 1062))
            .await?;
        lock.add(OceanColorMapping::new(s3a_olci, 29, 1267)).await?;
    }

    return Ok(());
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    // fetch environment variables
    #[cfg(feature = "postgres")]
    let database_connection_url = std::env::var("DATABASE_URL")?;

    let server_ip = std::env::var("SERVER_IP")?;

    let oceancolor_authorization = std::env::var("OCEANCOLOR_AUTHORIZATION")?;
    let oceancolor_job_timestep = std::env::var("OCEANCOLOR_JOB_TIMESTEP")?.parse::<u64>()?;
    let oceancolor_job_notfound = std::env::var("OCEANCOLOR_JOB_NOTFOUND")?.parse::<i64>()?;

    // config connection with database
    #[cfg(feature = "postgres")]
    let client = {
        let (client, connection) =
            tokio_postgres::connect(&database_connection_url, tokio_postgres::NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let client = Arc::new(Mutex::new(client));
        // TODO: migrate should be done by different tools in deploy time (k8s init containers)
        migrate(client.clone()).await?;

        client
    };

    // construct repositories
    #[cfg(not(feature = "postgres"))]
    let (
        satellite_repository,
        instrument_repository,
        satellite_instrument_repository,
        instrument_data_repository,
        oceancolor_mapping_repository,
    ) = {
        (
            create_inmemory_repository::<Satellite>(),
            create_inmemory_repository::<Instrument>(),
            create_inmemory_repository::<SatelliteInstrument>(),
            create_inmemory_repository::<InstrumentData>(),
            create_inmemory_repository::<OceanColorMapping>(),
        )
    };

    #[cfg(feature = "postgres")]
    let (
        satellite_repository,
        instrument_repository,
        satellite_instrument_repository,
        instrument_data_repository,
        oceancolor_mapping_repository,
    ) = {
        (
            create_postgres_repository::<Satellite>(client.clone(), "satellite"),
            create_postgres_repository::<Instrument>(client.clone(), "instrument"),
            create_postgres_repository::<SatelliteInstrument>(
                client.clone(),
                "satellite_instrument",
            ),
            create_postgres_repository::<InstrumentData>(client.clone(), "instrument_data"),
            create_postgres_repository::<OceanColorMapping>(client.clone(), "ocean_color_mapping"),
        )
    };

    // construct services
    let satellite_service = Arc::new(SatelliteServiceMock::new(satellite_repository.clone()));

    let celestrak_service = Arc::new(CelestrakServiceDefault::new());

    let instrument_data_service = Arc::new(InstrumentDataServiceDefault::new(
        satellite_instrument_repository.clone(),
        instrument_data_repository.clone(),
    ));

    let oceancolor_service = Arc::new(OceanColorServiceDefault::new(
        oceancolor_authorization,
        oceancolor_mapping_repository.clone(),
        instrument_data_service.clone(),
    ));

    // add test data
    add_test_data(
        celestrak_service.clone(),
        satellite_repository.clone(),
        instrument_repository.clone(),
        satellite_instrument_repository.clone(),
        oceancolor_mapping_repository.clone(),
    )
    .await?;

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
        celestrak_service,
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
