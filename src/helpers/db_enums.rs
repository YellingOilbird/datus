use std::str::FromStr;

use crate::*;
use diesel::AsExpression;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

/// Near indexer Action_kind type
#[derive(Debug, DbEnum, Clone, Copy, AsExpression, PartialEq, PartialOrd, Hash, Eq, Serialize, Deserialize)]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
#[DieselType = "Action_kind"]
#[PgType = "action_kind"]
pub enum ActionKind {
    CreateAccount,
    DeployContract,
    FunctionCall,
    Transfer,
    Stake,
    AddKey,
    DeleteKey,
    DeleteAccount,
}

impl FromStr for ActionKind {
    type Err = ();

    fn from_str(input: &str) -> Result<ActionKind, Self::Err> {
        match input {
            "CreateAccount"  => Ok(ActionKind::CreateAccount),
            "DeployContract" => Ok(ActionKind::DeployContract),
            "FunctionCall"   => Ok(ActionKind::FunctionCall),
            "Transfer"       => Ok(ActionKind::Transfer),
            "Stake"          => Ok(ActionKind::Stake),
            "AddKey"         => Ok(ActionKind::AddKey),
            "DeleteKey"      => Ok(ActionKind::DeleteKey),
            "DeleteAccount"  => Ok(ActionKind::DeleteAccount),
            _                => Err(())
        }
    }
}

/// Near indexer Execution_outcome_status type
#[derive(Debug, DbEnum, Clone, AsExpression, PartialEq, PartialOrd, Hash, Eq, Serialize, Deserialize)]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
#[DieselType = "Execution_outcome_status"]
#[PgType = "execution_outcome_status"]
pub enum ExecutionOutcomeStatus {
    Unknown,
    Failure,
    SuccessValue,
    SuccessReceiptId,
}

/// Near indexer State_change_reason_kind type
#[derive(Debug, Clone, AsExpression, DbEnum, PartialEq, PartialOrd)]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
#[DieselType = "State_change_reason_kind"]
#[PgType = "state_change_reason_kind"]
pub enum StateChangeReasonKind {
    TransactionProcessing,
    ActionReceiptProcessingStarted,
    ActionReceiptGasReward,
    ReceiptProcessing,
    PostponedReceipt,
    UpdatedDelayedReceipts,
    ValidatorAccountsUpdate,
    Migration,
    Resharding,
}