use serde::{Deserialize, Serialize};

use super::types::PassportElementError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetPassportDataErrors {
    pub user_id: i64,
    pub errors: Vec<PassportElementError>,
}
