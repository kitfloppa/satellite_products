use crate::{
    mapper,
    persistence::{model::instrument_data::InstrumentData, repository::Id},
    pub_fields,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub_fields! {
    #[derive(Deserialize, IntoParams)]
    struct GetBySatelliteIdRequest {
        #[param(value_type = i32)] // TODO: because Id not actuall type it's imposible for it implement ToSchema
        id: Id,
    }
}

#[derive(Serialize, ToSchema)]
pub struct InstrumentDataResponse {
    #[schema(value_type = i32)]
    id: Id,
    path: String,
}

mapper!(InstrumentData, InstrumentDataResponse, {
    path -> path,
});
