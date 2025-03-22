use std::sync::Arc;
use axum::{
    extract::State,
    response::{IntoResponse, Json},
};
use futures::future::join_all;
use sonic_defai_ai::ai::{prompt_gen, prompt_gen_combination, AI};
use sonic_defai_ai::types::{ SYSTEM};
use sonic_defai_defi::types::{ UserInfo};
use sonic_defai_defi::parser::{extract_wallet_asset_names, strategy_filter_by_depositable_asset, strategy_filter_by_risk};
use crate::parser::hashtag_num_parser;
use crate::types::{AppState, CombinationResponse, RecommendationResponse};

pub async fn recommend<AI_: AI + Send + Sync + 'static >(
    State(state): State<Arc<AppState<AI_>>>,
    Json(payload): Json<UserInfo>,
) -> impl IntoResponse {
    let asset_names = extract_wallet_asset_names(&payload);
    let risk_level = payload.risk;
    let risk_use = risk_level.clone();
    let stratigies = strategy_filter_by_depositable_asset( strategy_filter_by_risk(state.strategies.clone(), risk_level),asset_names );
    let wallet_balances: Option<Vec<sonic_defai_defi::types::Asset>> = payload.wallet_balance;

    if let Some(assets) = wallet_balances {

        let mut v = vec![];
        for asset in assets{
            let stratigies_clone = stratigies.clone();
            let risk_clone = risk_use.clone();
            let state_clone = state.clone();
            let handle = tokio::spawn(
                async move {
                    let user_prompt = prompt_gen(risk_clone, asset, stratigies_clone);
                    if let Ok(result) = state_clone.ai_client.query( SYSTEM , user_prompt.as_str()).await{
                             result
                         }
                         else{
                             "Wrong requests".to_string()
                         }
                }
            );
            v.push(handle);
        }

        let result = join_all(v).await;
        let result: Vec<String> = result.iter().map(|s|
            if let Ok(re) = s{
                re.clone()
            }
            else{
                "Error Occurred!".to_string()
            }
        ).collect();
        let index_list:Vec<_> =result.iter().map(|s| hashtag_num_parser(s) ).collect();

        let chosen_stratigies: Vec<_> = index_list.into_iter().map(|index|
            if let Some(st)  = state.strategies.get(index).clone(){
                st.clone()
            }
            else{
                state.strategies[0].clone()
            }
        ).collect();

        Json(RecommendationResponse {
            ai_responses: Some(result),
            strategies: Some(chosen_stratigies),
        })
    }
    else{
        Json(RecommendationResponse {
            ai_responses: None,
            strategies: None,
        })
    }
}

pub async fn combination<AI_: AI + Send + Sync + 'static >(
    State(state): State<Arc<AppState<AI_>>>,
    Json(payload): Json<UserInfo>,
) -> impl IntoResponse {
    let asset_names = extract_wallet_asset_names(&payload);
    let risk_level = payload.risk;
    let risk_use = risk_level.clone();
    let stratigies = strategy_filter_by_depositable_asset( strategy_filter_by_risk(state.strategies.clone(), risk_level),asset_names );
    let wallet_balances = payload.wallet_balance;


    if let Some(assets) = wallet_balances {
        let user_prompt = prompt_gen_combination(risk_use, assets, stratigies.clone()).await;
        let ai_response = if let Ok(result) = state.ai_client.query(SYSTEM, user_prompt.as_str()).await {
            result
        } else {
            "Wrong requests".to_string()
        };
        let index = hashtag_num_parser(&ai_response);

        let chosen_stratigy = if let Some(st) = state.strategies.get(index).clone() {
            st.clone()
        } else {
            state.strategies[0].clone()
        };


        Json(CombinationResponse {
            ai_responses: Some(ai_response),
            strategies: Some(chosen_stratigy),
        })
    } else {
        Json(CombinationResponse {
            ai_responses: None,
            strategies: None,
        })
    }
}
