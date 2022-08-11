use crate::helpers::{
    api_errors::ApiError,
    db_enums::*
};
use crate::db;
use crate::models::{
    ReportConfig,
    types::U128,
    utils::{
        AccountId, 
        args_vec_to_key_vec, 
        SYSTEM, AMOUNT
    }
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::schema::action_receipts__actions;
use crate::schema::execution_outcomes;

type FungibleTokenDeposited = HashMap<AccountId, HashMap<AccountId, u128>>;

#[derive(Queryable, Clone, Debug, PartialEq)]
pub struct ArgsReceivers {
    pub receiver_id: String,
    pub predecessor_account_id: String,
    pub args: serde_json::Value
}

/// `action_receipts__actions.args`. Used in `ContractInteractions`
/// which is simplify struct for all columns from `action_receipts__actions`
/// parsed to `DataReceipt`
#[derive(Serialize, Deserialize, Debug)]
pub struct ArgsFunctionCall {
    pub gas: Option<u128>,
    pub args_json: Option<HashMap<String, String>>,
    pub args_base64: Option<String>,
    pub deposit: Option<U128>,
    pub method_name: Option<String>
}
impl Default for ArgsFunctionCall {
    fn default() -> Self{
        Self {
            gas: Some(0),
            args_json: Some(HashMap::new()), 
            args_base64: Some("".into()),
            deposit: Some(U128(0)), 
            method_name: Some("".into())
        }
    }
}

#[derive(Serialize, Deserialize, Queryable, AsExpression, Debug)]
pub struct FungibleTokenDeposits {
    pub viewed_contracts: Vec<AccountId>,
    pub total_deposits_per_contract: FungibleTokenDeposited
}

impl FungibleTokenDeposits {
    pub fn get_deposits(config: ReportConfig) -> Result<Self, ApiError> {
        info!("Start processing deposits...");
        let connection = db::connection()?;
        let (from_timestamp, to_timestamp, limit) = config.unwrap()?;
        
        assert!(config.watched_tokens.is_some(),"ERR_WATCHED_TOKENS_REQUIRED");

        let result = action_receipts__actions::table
            .inner_join(execution_outcomes::table.on(
                action_receipts__actions::receipt_id.eq(execution_outcomes::receipt_id))
            )
            .limit(limit)
            .filter(action_receipts__actions::predecessor_account_id.not_like(SYSTEM.to_string()))
            .filter(action_receipts__actions::receiver_account_id.eq_any(config.contract_ids.clone()))
            .filter(action_receipts__actions::action_kind.eq(ActionKind::FunctionCall))
            .filter(execution_outcomes::status.ne(ExecutionOutcomeStatus::Failure))
            .filter(action_receipts__actions::block_timestamp.ge(from_timestamp))
            .filter(action_receipts__actions::block_timestamp.le(to_timestamp))
            .select((
                action_receipts__actions::receiver_account_id, 
                action_receipts__actions::predecessor_account_id,
                action_receipts__actions::args
            ))
            .load::<ArgsReceivers>(&connection);
        
        let args:Vec<ArgsReceivers> = match result {
            Ok(v) => {
                v
            },
            Err(e) => panic!("ERR_GETTING_TRANSACTIONS:{}", e),
        };

        info!("Preparing data to views...");
        
        let total_deposits_per_contract:FungibleTokenDeposited = processing_args(args, config.watched_tokens.unwrap());

        Ok(
            FungibleTokenDeposits {
                viewed_contracts: config.contract_ids.clone(),
                total_deposits_per_contract,
            }
        )
    }
}

fn processing_args(args: Vec<ArgsReceivers>, watched_tokens: Vec<AccountId>) -> FungibleTokenDeposited {
    let (arguments_vec, receivers) = args_vec_to_key_vec(args);
    let mut ft_maps: FungibleTokenDeposited = HashMap::new();
    for receiver in receivers.iter() {
        ft_maps.insert(receiver.clone(), HashMap::new());
    }
    for (receiver, action_args) in arguments_vec.iter() {
        let args_parsed:ArgsFunctionCall = serde_json::from_value(action_args.args.clone()).unwrap_or_default();
        let args_json = args_parsed.args_json.unwrap_or_default();
        let amount_ft = args_json.get(AMOUNT.into()).unwrap_or(&"0".into()).clone();
        let amount_ft_u128 = amount_ft.parse::<u128>().unwrap();
        // NEAR
        if args_parsed.deposit.is_some() && amount_ft_u128 == 0 {
            // working with args_parsed.deposit
        } else if amount_ft_u128 > 0 {
            // receiver ft deposits
            if watched_tokens.contains(&action_args.predecessor_account_id) {
                if let Some(x) = ft_maps.get_mut(receiver) {
                    if x.contains_key(&action_args.predecessor_account_id) {
                        *x.get_mut(&action_args.predecessor_account_id).unwrap() += amount_ft_u128;
                    } else {
                        x.insert(action_args.predecessor_account_id.clone(), amount_ft_u128);
                    }
                }
            }
        }
    }
    ft_maps
}