use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Donate(DonateMsg),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DonateMsg {}

#[derive(Serialize, Deserialize)]
pub struct QueryResp {
    message: String,
}

impl QueryResp {
    pub fn new<S>(msg: S) -> Self
    where
        S: ToString,
    {
        Self {
            message: msg.to_string(),
        }
    }
}
