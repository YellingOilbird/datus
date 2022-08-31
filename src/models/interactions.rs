use crate::helpers::api_errors::ApiError;
use crate::db;
use crate::schema::action_receipts__actions::dsl::*;
use crate::helpers::db_enums::ActionKind;
use crate::models::ReportConfig;
use crate::models::utils::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;


#[derive(Serialize, Deserialize, Queryable, AsExpression, Debug)]
pub struct AccountsInteracted {
    pub total_interacted_accounts: u64,
    pub viewed_contracts: Vec<AccountId>,
    pub interacted_accounts: Vec<(AccountId, u64)>
}

impl AccountsInteracted {
    pub fn get_interactions(config: ReportConfig) -> Result<Self, ApiError> {
        let connection = db::connection()?;
        let (from_timestamp, to_timestamp, limit) = config.unwrap()?;
        // TODO - add execution outcome statuses
        // filtered by receivers (our contracts) and timestamp
        // if from_datetime == None - from GENESIS block
        let filtered = action_receipts__actions
            .filter(receiver_account_id.eq_any(config.contract_ids.clone()))
            .filter(predecessor_account_id.not_like(SYSTEM.to_string()))
            .filter(predecessor_account_id.ne_all(config.contract_ids.clone()))
            .filter(action_kind.eq(ActionKind::FunctionCall))
            .filter(block_timestamp.ge(from_timestamp))
            .filter(block_timestamp.le(to_timestamp));
            //.order_by(block_timestamp.desc());

        // report accounts
        // unique interacted with required contracts user accounts
        let mut interacted_accounts:Vec<AccountId> = filtered
            .limit(limit)       
            .select(predecessor_account_id)
            .order_by(predecessor_account_id.desc())
            .load::<AccountId>(&connection)?;
        
        let mut accounts_map:Vec<(AccountId, u64)> = interacted_accounts
            .iter()
            .map(
                |account| 
                count_account_entries(
                    interacted_accounts.clone(), 
                    account
                )
            )
            .collect();
        interacted_accounts.dedup();

        accounts_map.sort_by(|acc1, acc2| acc2.1.cmp(&acc1.1));
        accounts_map.drain(5..);

        let total_interacted_accounts = interacted_accounts.len() as u64;
        info!("Total interacted with contracts {:?} users: {:?} ", config.contract_ids.clone(), total_interacted_accounts);

        Ok(AccountsInteracted {
            total_interacted_accounts,
            viewed_contracts: config.contract_ids.clone(),
            interacted_accounts: accounts_map
        })
    }
}