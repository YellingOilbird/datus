use std::str::FromStr;
use crate::models::{ArgsReceivers, Transaction};
use std::collections::HashMap;
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::ops::Range;

pub(crate) const NEAR_GENESIS_TS:&str = "1595350551591948000";
pub(crate) const SYSTEM:&str = "system";
pub(crate) const AMOUNT:&str = "amount";
const ONE_DAY_SECONDS:u64 = 60 * 60 * 24;
const TS_MAX:u64 = u64::MAX / 1_000_000;
/// uncheked for a while
pub(crate) type AccountId = String;

/// DateTime::<Utc> - BigDecimal (its rounded to seconds at the moment)
pub(crate) fn convert_datetime_to_timestamp(datetime: DateTime<Utc>) -> BigDecimal {
    let timestamp = datetime.timestamp_nanos().to_string();
    BigDecimal::from_str(&timestamp).unwrap()
}

/// BigDecimal - u64
pub(crate) fn big_decimal_to_timestamp(ts: BigDecimal) -> u64 {
    let ts = ts.to_string();
    ts.parse().unwrap()
}

pub(crate) fn convert_timestamp_to_datetime(ts: u64) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(ts.try_into().unwrap(), 0), Utc)
}

pub(crate) fn count_account_entries(v: Vec<AccountId>, acc: &AccountId) -> (AccountId, u64) {
    (
        acc.clone(),
        v.iter().filter(|&account| account == acc).count() as u64
    )
}

/// MILLI_SEC timestamp (u64) -> BigDecimal
pub(crate) fn timestamp_to_big_decimal(ts: u64) -> BigDecimal {
    assert!(ts < TS_MAX, "Avoid math overflow");
    let ts = (ts * 10u64.pow(6)).to_string();
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

pub(crate) fn processed_daily_transactions(mut v: Vec<u64>) -> Vec<(DateTime<Utc>, u64)> {
    // sort timestamps
    v.sort_unstable();

    let first_timestamp = v[0];
    let last_timestamp = v[v.len()-1];

    let from = ( first_timestamp / ONE_DAY_SECONDS ) * ONE_DAY_SECONDS;
    let to = ( last_timestamp / ONE_DAY_SECONDS + 1) * ONE_DAY_SECONDS;

    let duration_in_days = ( (to - from) / ONE_DAY_SECONDS ) as usize;
    let mut ranges: Vec<u64> = vec![0; duration_in_days];

    for i in 0..duration_in_days {
        ranges[i] = from + ONE_DAY_SECONDS * (i as u64);
    }

    info!("ranges last {:?} to {} (must be relative equal)",ranges[duration_in_days - 1], to);

    let daily_ranges = ranges.iter()
        .map(|&timestamp_00| {
            timestamp_00..(timestamp_00 + ONE_DAY_SECONDS)
        })
        .collect::<Vec<Range<u64>>>();

    info!("ranges {:#?}", daily_ranges);

    let mut result: HashMap<usize, u64> = daily_ranges.iter().enumerate()
        .map(|(day,_)| (day, 0))
        .collect();

    info!("setting result {:#?}", result);
    
    for (day, daily_range) in daily_ranges.iter().enumerate() {
        for timestamp in &v {
            if *timestamp >= daily_range.start && *timestamp <= daily_range.end {
                *result.get_mut(&day).unwrap() += 1;
            };
        }
    }
    
    let mut vec_result: Vec<_> = result.into_iter().collect();
    vec_result.sort_by(|a,b| a.0.cmp(&b.0));

    vec_result.iter().enumerate()
        .map(|(index,(day,num))| {
            info!("day -  {}, index - {}, ranges one - {}", day, index, ranges[*day]);
            let datetime_utc = convert_timestamp_to_datetime(ranges[*day]);
            (datetime_utc, *num)
        })
        .collect()
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