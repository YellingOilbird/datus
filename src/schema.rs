table! {
    use diesel::sql_types::*;
    use crate::helpers::db_enums::*;
    account_changes (block_timestamp, chunk_index_in_block, index_in_chunk) {
        account_id -> Text,
        block_timestamp -> Numeric,
        block_hash -> Text,
        caused_by_transaction_hash -> Nullable<Text>,
        caused_by_receipt_id -> Nullable<Text>,
        update_reason -> State_change_reason_kind,
        nonstaked_balance -> Numeric,
        staked_balance -> Numeric,
        storage_usage -> Numeric,
        chunk_index_in_block -> Integer,
        index_in_chunk -> Integer,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::helpers::db_enums::*;
    #[allow(non_snake_case)]
    action_receipts__actions (receipt_id, chunk_index_in_block, index_in_chunk) {
        block_hash -> Text,
        block_timestamp -> Numeric,
        receipt_id -> Text,
        action_kind -> Action_kind,
        args -> Jsonb,
        predecessor_account_id -> Text,
        receiver_account_id -> Text,
        chunk_index_in_block -> Integer,
        index_in_chunk -> Integer,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    action_receipts (receipt_id) {
        receipt_id -> Text,
        block_hash -> Text,
        chunk_hash -> Text,
        block_timestamp -> Numeric,
        chunk_index_in_block -> Int4,
        receipt_index_in_chunk -> Int4,
        predecessor_account_id -> Text,
        receiver_account_id -> Text,
        originated_from_transaction_hash -> Text,
        signer_account_id -> Text,
        signer_public_key -> Text,
        gas_price -> Numeric,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    accounts (account_id, created_by_block_height) {
        account_id -> Text,
        created_by_receipt_id -> Nullable<Text>,
        deleted_by_receipt_id -> Nullable<Text>,
        created_by_block_height -> Numeric,
        deleted_by_block_height -> Numeric,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::helpers::db_enums::*;
    #[allow(non_snake_case)]
    transactions (transaction_hash) {
        transaction_hash -> Text,
        block_hash -> Text,
        chunk_hash -> Text,
        block_timestamp -> Numeric,
        chunk_index_in_block -> Int4,
        index_in_chunk -> Int4,
        signer_account_id -> Text,
        signer_public_key -> Text,
        nonce -> Numeric,
        receiver_account_id -> Text,
        signature -> Text,
        status -> Execution_outcome_status,
        converted_into_receipt_id -> Text,
        receipt_conversion_gas_burnt -> Nullable<Numeric>,
        receipt_conversion_tokens_burnt -> Nullable<Numeric>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::helpers::db_enums::*;
    #[allow(non_snake_case)]
    execution_outcomes (receipt_id) {
        receipt_id -> Text,
        block_hash -> Text,
        block_timestamp -> Numeric,
        chunk_index_in_block -> Int4,
        index_in_chunk -> Int4,
        gas_burnt -> Numeric,
        tokens_burnt -> Numeric,
        executor_account_id -> Text,
        status -> Execution_outcome_status,
    }
}


allow_tables_to_appear_in_same_query!(transactions, action_receipts__actions, action_receipts, execution_outcomes);
joinable!(action_receipts__actions -> action_receipts (receipt_id));
joinable!(action_receipts -> transactions (originated_from_transaction_hash));
joinable!(execution_outcomes -> action_receipts (receipt_id));