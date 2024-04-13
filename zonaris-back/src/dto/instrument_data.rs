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
        id: Id,
    }
}

#[derive(Serialize, ToSchema)]
pub struct InstrumentDataResponse {
    id: Id,
}

mapper!(InstrumentData, InstrumentDataResponse, {});

pub_fields! {
    #[derive(Deserialize, IntoParams)]
    struct GetAssetRequest {
        id: Id,
    }
}
