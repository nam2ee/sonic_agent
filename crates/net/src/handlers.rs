use std::sync::Arc;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use sonic_defai_ai::ai::{prompt_gen, AI};
use sonic_defai_ai::types::{ SYSTEM};
use sonic_defai_defi::types::{Risk, UserInfo};
use sonic_defai_defi::parser::strategy_filter;
use crate::types::{AppState};
async fn recommend<AI_: AI>(
    State(state): State<Arc<AppState<AI_>>>,
    Json(payload): Json<UserInfo>,
) -> Vec<String> {

    let risk_level = payload.risk;
    let risk_use = risk_level.clone();
    let stratigies = strategy_filter(state.strategies.clone(), risk_level);

    let wallet_balances = payload.wallet_balance;
    if let Some(assets) = wallet_balances {

        assets.into_iter().map( async | asset| {
            let user_prompt = prompt_gen(risk_use, asset, stratigies.clone());
            state.ai_client.query( SYSTEM , user_prompt.into()).await
        }).collect()

    }
    else{
        vec!["Wrong requests".to_string()]
    }
}