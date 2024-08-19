use crate::bot::Bot;

pub mod payload;
pub mod types;
use anyhow::Result;
use types::PassportElementError;
impl Bot {
    pub async fn set_passport_data_errors(
        &self,
        payload: &Vec<PassportElementError>,
    ) -> Result<bool> {
        self.call_api_json("setPassportDataErrors", payload).await
    }
}
