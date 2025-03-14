use std::sync::Arc;
use axum::{
    extract::State,
    response::{IntoResponse, Json},
};
use futures::future::join_all;
use sonic_defai_ai::ai::{prompt_gen, AI};
use sonic_defai_ai::types::{ SYSTEM};
use sonic_defai_defi::types::{ UserInfo};
use sonic_defai_defi::parser::strategy_filter;
use crate::types::{AppState};

pub async fn recommend<AI_: AI>(
    State(state): State<Arc<AppState<AI_>>>,
    Json(payload): Json<UserInfo>,
) -> impl IntoResponse {

    let risk_level = payload.risk;
    let risk_use = risk_level.clone();
    let stratigies = strategy_filter(state.strategies.clone(), risk_level);

    let wallet_balances = payload.wallet_balance;
    if let Some(assets) = wallet_balances {

        let v:Vec<_> = assets.into_iter().map( async | asset| {
            let user_prompt = prompt_gen(risk_use.clone(), asset, stratigies.clone());
            if let Ok(result) = state.ai_client.query( SYSTEM , user_prompt.as_str()).await{
                result
            }
            else{
                "Wrong requests".to_string()
            }
        }).collect();

        let result = join_all(v).await;

        Json(result)
    }
    else{
        Json(vec!["Wrong requests".to_string()])
    }
}