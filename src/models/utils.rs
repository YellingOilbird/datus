use std::str::FromStr;
use crate::models::{ArgsReceivers, Transaction};
use std::collections::HashMap;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

pub(crate) const NEAR_GENESIS_TS:&str = "1595350551591948000";
pub(crate) const SYSTEM:&str = "system";
pub(crate) const AMOUNT:&str = "amount";


/// uncheked for a while
pub(crate) type AccountId = String;

/// DateTime::<Utc> - BigDecimal (its rounded to seconds at the moment)
pub(crate) fn convert_datetime_to_timestamp(datetime: DateTime<Utc>) -> BigDecimal {
    let timestamp = datetime.timestamp_nanos().to_string();
    BigDecimal::from_str(&timestamp).unwrap()
}

#[allow(unused)]
/// BigDecimal - u64
pub(crate) fn big_decimal_to_timestamp(ts: BigDecimal) -> u64 {
    let ts = ts.to_string();
    ts.parse().unwrap()
}

/// u64 -> BigDecimal
pub(crate) fn timestamp_to_big_decimal(ts: u64) -> BigDecimal {
    let ts = ts.to_string();
    BigDecimal::from_str(&ts).unwrap()
}

/// Transactions to JSON Repost parameters
pub(crate) fn processed_transactions(v: Vec<Transaction>) -> (u64, HashMap<String, u64>) {
    let count = v.iter().count();
    let receivers:Vec<String> = v
        .iter()
        .map(|v| v.receiver_account_id.clone())
        .collect();
    let mut receiver_to_transaction: Vec<(String, &Transaction)> = vec![];
    for (index, receiver) in receivers.iter().enumerate() {
        receiver_to_transaction.push((receiver.clone(), &v[index]))
    }
    let mut map: HashMap<String, u64> = receiver_to_transaction
        .iter()
        .map(|x| (x.0.clone(),0))
        .collect();
    for (acc,_item) in receiver_to_transaction.iter() {
        *map.get_mut(acc).unwrap() += 1;
    }
    (count as u64, map)
}

pub(crate) fn args_vec_to_key_vec(struct_vec: Vec<ArgsReceivers>) -> (Vec<(String, ArgsReceivers)>, Vec<String>) {
    let mut receivers:Vec<String> = struct_vec
        .iter()
        .map(|v| v.receiver_id.clone())
        .collect();
    let mut result: Vec<(String, ArgsReceivers)> = vec![];

    assert!(receivers.len() == struct_vec.len());

    for (index, receiver) in receivers.iter().enumerate() {
        result.push((receiver.clone(), struct_vec[index].clone()));
    }
    receivers.sort_unstable();
    receivers.dedup();
    (result, receivers)
}