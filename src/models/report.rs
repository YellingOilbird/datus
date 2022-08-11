use crate::models::utils::*; 
use bigdecimal::BigDecimal;
use chrono::Utc;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

use crate::helpers::api_errors::ApiError;

#[derive(Serialize, Deserialize)]
pub struct ReportConfig {
    pub limit: Option<i64>,
    pub contract_ids: Vec<AccountId>,
    pub from_timestamp: Option<u64>,
    pub to_timestamp: Option<u64>,
    pub watched_tokens: Option<Vec<AccountId>> 
}

impl ReportConfig {
    pub fn unwrap(&self) -> Result<(BigDecimal, BigDecimal, i64), ApiError> {
        let from_timestamp = match self.from_timestamp {
            Some(ts) => timestamp_to_big_decimal(ts),
            None => BigDecimal::from_str(&NEAR_GENESIS_TS).unwrap(),
        };
        let to_timestamp = match self.to_timestamp {
            Some(ts) => timestamp_to_big_decimal(ts),
            None => {
                let current_time = Utc::now();
                convert_datetime_to_timestamp(current_time)
            },
        };
        let _limit:i64 = if let Some(limit) = self.limit {
            limit
        } else {
            i64::MAX
        }; 
        Ok((from_timestamp, to_timestamp, _limit))
    }
}