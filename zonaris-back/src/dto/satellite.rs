use crate::{
    mapper,
    persistence::{model::satellite::Satellite, repository::Id},
};

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct SatelliteResponse {
    #[schema(value_type = i32)]
    id: Id,
    name: String,
    tle1: String,
    tle2: String,
}

mapper!(Satellite, SatelliteResponse, {
    name -> name,
    tle1 -> tle1,
    tle2 -> tle2,
});
