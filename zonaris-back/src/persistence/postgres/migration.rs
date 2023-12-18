// TODO: normal migrations
// TODO: delete IF NOT EXISTS

use anyhow::Result;

use super::Client;

pub async fn migrate(client: Client) -> Result<()> {
    init_db(&client).await?;

    return Ok(());
}

pub async fn init_db(client: &Client) -> Result<()> {
    let mut client = client.lock().await;
    let transaction = client.transaction().await?;

    let statement = "CREATE TABLE IF NOT EXISTS satellite
    (
        id SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL,
        tle1 VARCHAR NOT NULL,
        tle2 VARCHAR NOT NULL
    );";
    transaction.execute(statement, &[]).await?;

    let statement = "CREATE TABLE IF NOT EXISTS instrument
    (
        id SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL
    );";
    transaction.execute(statement, &[]).await?;

    let statement = "CREATE TABLE IF NOT EXISTS satellite_instrument
    (
        id SERIAL PRIMARY KEY,
        satellite_id INTEGER NOT NULL REFERENCES satellite,
        instrument_id INTEGER NOT NULL REFERENCES instrument
    );";
    transaction.execute(statement, &[]).await?;

    let statement = "CREATE TABLE IF NOT EXISTS ocean_color_mapping
    (
        id SERIAL PRIMARY KEY,
        satellite_instrument_id INTEGER NOT NULL REFERENCES satellite_instrument,
        sensor_id INTEGER NOT NULL,
        data_id INTEGER NOT NULL
    );";
    transaction.execute(statement, &[]).await?;

    let statement = "CREATE TABLE IF NOT EXISTS instrument_data
    (
        id SERIAL PRIMARY KEY,
        satellite_instrument_id INTEGER NOT NULL REFERENCES satellite_instrument,
        path VARCHAR NOT NULL
    );";
    transaction.execute(statement, &[]).await?;

    transaction.commit().await?;
    return Ok(());
}
