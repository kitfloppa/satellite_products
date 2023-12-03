use crate::persistence::repository::Id;
use serde::Deserialize;

crate::pub_fields! {
    #[derive(Deserialize)]
    struct GetBySatelliteIdRequest {
        id: Id,
    }
}
