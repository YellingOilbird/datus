use crate::models::{AccountsInteracted, Transactions, ReportConfig, FungibleTokenDeposits};
use crate::helpers::api_errors::ApiError;
use actix_web::{HttpResponse, Responder, get, post, web};

#[post("/totalAccountsInteracted")]
async fn get_accounts_interacted(params: web::Json<ReportConfig>) -> Result<HttpResponse, ApiError> {
    let result = AccountsInteracted::get_interactions(params.into_inner())?;
    Ok(HttpResponse::Ok().json(result))
}

#[post("/totalTransactions")]
async fn get_transactions(params: web::Json<ReportConfig>) -> Result<HttpResponse, ApiError> {
    let result = Transactions::get_transactions(params.into_inner())?;
    Ok(HttpResponse::Ok().json(result))
}

#[post("/totalFTDeposits")]
async fn get_ft_deposits(params: web::Json<ReportConfig>) -> Result<HttpResponse, ApiError> {
    let result = FungibleTokenDeposits::get_deposits(params.into_inner())?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong!")
}

// #[get("/receipts/{params}")]
// async fn receipts(params: web::Path<String>) -> Result<HttpResponse, ApiError> {
//     let params_split:Vec<_> = params.split("@").collect();
//     let params_vec:Vec<String> = params_split.iter().map(|&p| p.to_string()).collect();

//     let params = TestingParams::from(params_vec);
//     let result = Testing::find(&params)?;
//     Ok(HttpResponse::Ok().json(result))
// }

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_accounts_interacted);
    cfg.service(ping);
    cfg.service(get_transactions);
    cfg.service(get_ft_deposits);
}