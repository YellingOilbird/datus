use crate::helpers::api_errors::ApiError;
use crate::db;
use crate::helpers::db_enums::*;
use crate::models::ReportConfig;
use crate::models::utils::*;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{DateTime, Utc};
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use std::collections::HashMap;
use crate::schema::transactions;
use crate::schema::action_receipts;
use crate::schema::execution_outcomes;
use crate::models::utils::SYSTEM;

#[derive(Queryable, Clone, Debug, PartialEq)]
pub struct Transaction {
    pub transaction_hash: String,
    pub receipt_id: String,
    pub receiver_account_id: String,
    pub block_timestamp: BigDecimal,
}

#[derive(Serialize, Deserialize, Queryable, AsExpression, Debug)]
pub struct Transactions {
    pub total_transactions: u64,
    pub viewed_contracts: Vec<AccountId>,
    pub total_transactions_per_contract: HashMap<AccountId, u64>
}

#[derive(Serialize, Deserialize, Queryable, AsExpression, Debug)]
pub struct DailyTransactions {
    pub daily_transactions: Vec<(DateTime<Utc>, u64)>,
    pub total_transactions: u64
}

impl Transactions {
    pub fn get_transactions(config: ReportConfig) -> Result<Self, ApiError> {
        info!("Start processing transactions...");
        let connection = db::connection()?;
        let (from_timestamp, to_timestamp, limit) = config.unwrap()?;
        let result = transactions::table
            .inner_join(action_receipts::table.on(
                action_receipts::originated_from_transaction_hash.eq(transactions::transaction_hash))
            )
            .inner_join(execution_outcomes::table.on(
                execution_outcomes::receipt_id.eq(transactions::converted_into_receipt_id)
            ))
            .limit(limit)
            .filter(action_receipts::predecessor_account_id.not_like(SYSTEM.to_string()))
            .filter(execution_outcomes::status.ne(ExecutionOutcomeStatus::Failure))
            .filter(transactions::signer_account_id.ne_all(config.contract_ids.clone()))
            .filter(transactions::receiver_account_id.eq_any(config.contract_ids.clone()))
            .filter(transactions::block_timestamp.ge(from_timestamp))
            .filter(transactions::block_timestamp.le(to_timestamp))
            .select((
                transactions::transaction_hash, 
                action_receipts::receipt_id,
                transactions::receiver_account_id, 
                transactions::block_timestamp, 
            ))
            .load::<Transaction>(&connection);
        
        let transactions:Vec<Transaction> = match result {
            Ok(v) => {
                v
            },
            Err(e) => panic!("ERR_GETTING_TRANSACTIONS:{}", e),
        };

        info!("Preparing data to views...");
        
        let (total_transactions, total_transactions_per_contract) = processed_transactions(transactions);

        Ok(
            Transactions {
                total_transactions,
                viewed_contracts: config.contract_ids.clone(),
                total_transactions_per_contract,
            }
        )
    }
}

impl DailyTransactions {
    pub fn get_transactions(config: ReportConfig) -> Result<Self, ApiError> {
        info!("Start processing transactions...");
        let connection = db::connection()?;
        let (from_timestamp, to_timestamp, limit) = config.unwrap()?;
        let result = transactions::table
            .inner_join(action_receipts::table.on(
                action_receipts::originated_from_transaction_hash.eq(transactions::transaction_hash))
            )
            .inner_join(execution_outcomes::table.on(
                execution_outcomes::receipt_id.eq(transactions::converted_into_receipt_id)
            ))
            .limit(limit)
            .filter(action_receipts::predecessor_account_id.not_like(SYSTEM.to_string()))
            .filter(execution_outcomes::status.ne(ExecutionOutcomeStatus::Failure))
            .filter(transactions::signer_account_id.ne_all(config.contract_ids.clone()))
            .filter(transactions::receiver_account_id.eq_any(config.contract_ids.clone()))
            .filter(transactions::block_timestamp.ge(from_timestamp))
            .filter(transactions::block_timestamp.le(to_timestamp))
            .select(transactions::block_timestamp)
            .load::<BigDecimal>(&connection);
        
        let transactions:Vec<u64> = match result {
            Ok(v) => {
                info!("Debug timestamps...{:?} ", v.clone());
                v.iter()
                    .map(|x| {
                        let nanos = x.clone().to_u64().unwrap();
                        nanos / 10u64.pow(9u32)
                    })
                    .collect() 
            },
            Err(e) => panic!("ERR_GETTING_TRANSACTIONS:{}", e),
        };

        info!("Preparing data to views...{:?} ", transactions.clone());
        
        let daily_transactions = processed_daily_transactions(transactions.clone());

        Ok(
            DailyTransactions {
                daily_transactions,
                total_transactions: transactions.len() as u64
            }
        )
    }
}