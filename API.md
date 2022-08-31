1. Setting up `ReportConfig` with given parameters:
```rust
pub struct ReportConfig {
    pub limit: Option<i64>,
    pub contract_ids: Vec<AccountId>,
    pub from_timestamp: Option<u64>,
    pub to_timestamp: Option<u64>,
    pub watched_tokens: Option<Vec<AccountId>> 
}
```
- `Option` parameters may not set. If this has no param it will be default variables:
    - `limit`          = `i64::MAX`
    - `from_timestamp` = `NEAR_GENESIS_TIME`
    - `to_timestamp`   = `Utc::now()` converted to timestamp
   *- `watched_tokens` doesn't have default value, but REQUIRED for retrieving POST::totalFTDeposits*

### API

```rust
/// Ping-Pong
#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong!")
}

/// Interactions
pub struct AccountsInteracted {
    pub total_interacted_accounts: u64, 
    pub viewed_contracts: Vec<AccountId>, //from ReportConfig.contract_ids directly
    pub interacted_accounts: Vec<AccountId> //from DB query
}
/// POST::totalAccountsInteracted
/// where `get_interactions` - query with config filters (./src/models/interactions)
#[post("/totalAccountsInteracted")]
async fn get_accounts_interacted(params: ReportConfig) -> Result<HttpResponse, ApiError> {
    let result = AccountsInteracted::get_interactions(params)?;
    Ok(HttpResponse::Ok().json(result))
}

/// Transactions
pub struct Transactions {
    pub total_transactions: u64,
    pub viewed_contracts: Vec<AccountId>, //from ReportConfig.contract_ids directly
    // this transformed from vector(Array) with query result to map
    // via `fn processed_transactions` from (./src/models/utils)
    pub total_transactions_per_contract: HashMap<AccountId, u64> //from DB query 
}
/// POST::totalTransactions
/// where `get_transactions` - query with config filters (./src/models/transactions)
#[post("/totalTransactions")]
async fn get_transactions(params: ReportConfig) -> Result<HttpResponse, ApiError> {
    let result = Transactions::get_transactions(params);
    Ok(HttpResponse::Ok().json(result))
}
/// "total_deposits_per_contract": {
///     "p3-meta.cheddar.near": {
///         "token.cheddar.near": 209077706899999997977180864000
///     }
/// }
type FungibleTokenDeposited = HashMap<AccountId, HashMap<AccountId, u128>>;
/// FT Deposits to viewed contracts
pub struct FungibleTokenDeposits {
    pub viewed_contracts: Vec<AccountId>,
    // Map of viewed_contract:{token_contract:deposit} for each viewed_contract
    pub total_deposits_per_contract: FungibleTokenDeposited
}
/// POST::totalFTDeposits
/// where `get_deposits` - query with config filters (./src/models/ft_deposits)
#[post("/totalFTDeposits")]
async fn get_ft_deposits(params: ReportConfig) -> Result<HttpResponse, ApiError> {
    let result = FungibleTokenDeposits::get_deposits(params)?;
    Ok(HttpResponse::Ok().json(result))
}
/// daily_transactions : [(date, number)...(date, number)]
/// "daily_transactions": {
///     "p3-meta.cheddar.near": {
///         "token.cheddar.near": 209077706899999997977180864000
///     }
/// }
#[derive(Serialize, Deserialize, Queryable, AsExpression, Debug)]
pub struct DailyTransactions {
    pub daily_transactions: Vec<(DateTime<Utc>, u64)>,
    pub total_transactions: u64
}
/// POST::dailyTransactions
#[post("/dailyTransactions")]
async fn get_daily_transactions(params: web::Json<ReportConfig>) -> Result<HttpResponse, ApiError> {
    let result = DailyTransactions::get_transactions(params.into_inner())?;
    Ok(HttpResponse::Ok().json(result))
}

```