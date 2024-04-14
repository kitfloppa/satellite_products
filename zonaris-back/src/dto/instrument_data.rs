use crate::persistence::repository::HasId;
use crate::{
    mapper,
    persistence::{model::instrument_data::InstrumentData, repository::Id},
};
use serde::{Deserialize, Serialize};
use table_macro::Property;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams, Property)]
pub struct GetBySatelliteIdRequest {
    id: Id,
}

#[derive(Serialize, ToSchema)]
pub struct InstrumentDataResponse {
    id: Id,
}

mapper!(InstrumentData, InstrumentDataResponse, {});

#[derive(Deserialize, IntoParams, Property)]
pub struct GetAssetRequest {
    id: Id,
}
