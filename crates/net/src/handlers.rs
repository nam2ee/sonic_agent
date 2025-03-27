use std::num::ParseFloatError;
use std::ops::Add;
use std::sync::Arc;
use axum::{
    extract::State,
    response::{IntoResponse, Json},
};
use futures::future::join_all;
use sonic_defai_ai::ai::{prompt_gen, prompt_gen_combination, AI};
use sonic_defai_ai::types::{ SYSTEM};
use sonic_defai_defi::types::{Risk, Strategy, UserInfo};
use sonic_defai_defi::parser::{extract_wallet_asset_names, strategy_filter_by_depositable_asset, strategy_filter_by_risk};
use crate::parser::{hashtag_num_parser, parse_asset_info, parse_asset_single};
use crate::types::{AppState, CombinationResponse, CombinationResponse_pre, RecommendationResponse, StrategyRecommendation};

pub async fn recommend<AI_: AI + Send + Sync + 'static>(
    State(state): State<Arc<AppState<AI_>>>,
    Json(payload): Json<UserInfo>,
) -> impl IntoResponse {
    let asset_names = extract_wallet_asset_names(&payload);
    let risk_level = payload.risk;
    let strategies = strategy_filter_by_depositable_asset(
        strategy_filter_by_risk(state.strategies.clone(), risk_level.clone()),
        asset_names
    );

    let mut recommendations = Vec::new();
    let mut ai_response_vec = Vec::new();

    if let Some(assets) = payload.wallet_balance {
        let mut handles = vec![];


        for asset in assets {
            let strategies_clone = strategies.clone();
            let risk_clone = risk_level.clone();
            let state_clone = state.clone();

            let handle = tokio::spawn(async move {
                process_asset(state_clone, risk_clone, asset, strategies_clone).await
            });

            handles.push(handle);
        }


        let results = join_all(handles).await;

        for result in results {
            if let Ok(Some((ai_response, strategy, used_asset))) = result {
                ai_response_vec.push(ai_response);
                recommendations.push(StrategyRecommendation {
                    strategy: strategy.clone(),
                    used_assets: vec![used_asset],
                    vault_address: strategy.address.clone(),
                });
            }
        }
    }

    Json(CombinationResponse {
        ai_responses: ai_response_vec,
        recommendations,
    })
}

async fn process_asset<AI_: AI + Send + Sync + 'static>(
    state: Arc<AppState<AI_>>,
    risk_level: Risk,
    asset: sonic_defai_defi::types::Asset,
    filtered_strategies: Vec<Strategy>,
) -> Option<(String, Strategy, (String, f64))> {
    let user_prompt = prompt_gen(risk_level, asset.clone(), filtered_strategies).await;

    match state.ai_client.query(SYSTEM, user_prompt.as_str()).await {
        Ok(result) => {
            let strategy_index = hashtag_num_parser(&result);
            let asset_info = parse_asset_single(&result);


            if asset_info.is_empty() {
                return None;
            }

            let asset_name = asset_info[0].0.to_lowercase();
            let amount_str = asset_info[0].1.as_str();

            let strategy = state.strategies.get(strategy_index).cloned()?;

            let depositable_assets_lower: Vec<String> = strategy.depositable_asset
                .iter()
                .map(|s| s.to_lowercase())
                .collect();

            if !depositable_assets_lower.contains(&asset_name) {
                return None;
            }



            match amount_str.parse::<f64>() {
                Ok(amount) => {
                    if asset.balance < amount{
                        Some((result, strategy, (asset_name, asset.balance)))
                    }
                    else{
                        Some((result, strategy, (asset_name, amount)))
                    }

                },
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}





pub async fn combination<AI_: AI + Send + Sync + 'static>(
    State(state): State<Arc<AppState<AI_>>>,
    Json(payload): Json<UserInfo>,
) -> impl IntoResponse {
    let asset_names = extract_wallet_asset_names(&payload);
    let risk_level = payload.risk;
    let mut remaining_assets = payload.wallet_balance.unwrap_or_default();
    let mut recommendations = Vec::new();
    let mut ai_response_vec = Vec::new();
    let mut loop_counter = 0;

    let filtered_strategies = strategy_filter_by_depositable_asset(
        strategy_filter_by_risk(state.strategies.clone(), risk_level.clone()),
        asset_names
    );


    while !remaining_assets.is_empty() && !filtered_strategies.is_empty() {
        if loop_counter >= 4 || remaining_assets.iter().all(|a| a.balance < 0.000001) {
            break;
        }
        loop_counter += 1;
        let user_prompt = prompt_gen_combination(
            risk_level.clone(),
            remaining_assets.clone(),
            filtered_strategies.clone()
        ).await;

        let ai_response = match state.ai_client.query(SYSTEM, user_prompt.as_str()).await {
            Ok(result) => result,
            Err(_) => break,
        };

        let strategy_index = hashtag_num_parser(&ai_response);
        let asset_info = parse_asset_info(&ai_response);


        if let Some(strategy) = state.strategies.get(strategy_index).cloned() {

            if asset_info.len() == 1 {
                let asset_name = asset_info[0].0.to_lowercase();
                let depositable_assets_lower: Vec<String> = strategy.depositable_asset
                    .iter()
                    .map(|s| s.to_lowercase())
                    .collect();

                if !depositable_assets_lower.contains(&asset_name) {
                    continue;
                }
            }
            else if asset_info.len() == 2 {

                let lp_asset_1 = asset_info[0].0.to_lowercase();
                let lp_asset_2 = asset_info[1].0.to_lowercase();

                let pair1 = format!("lp({},{})", lp_asset_1, lp_asset_2);
                let pair2 = format!("lp({},{})", lp_asset_2, lp_asset_1);

                let depositable_assets_lower: Vec<String> = strategy.depositable_asset
                    .iter()
                    .map(|s| s.to_lowercase())
                    .collect();

                if !depositable_assets_lower.contains(&pair1) && !depositable_assets_lower.contains(&pair2) && !( depositable_assets_lower.contains(&lp_asset_2) && depositable_assets_lower.contains(&lp_asset_1) ) {
                    continue;
                }
            }
            else{
                continue;
            }

            let mut used_assets = Vec::new();

            for (asset_name, amount_str) in asset_info {

                if let Ok(amount) = amount_str.parse::<f64>() {
                    if let Some(pos) = remaining_assets.iter().position(|a| a.name == asset_name) {
                        if remaining_assets[pos].balance >= amount {
                            remaining_assets[pos].balance -= amount;
                            used_assets.push((asset_name, amount));
                        }
                        else{
                            used_assets.push((asset_name, remaining_assets[pos].balance));
                            remaining_assets[pos].balance = 0_f64;
                        }
                    }
                } //TODO! : Error handling
            }

            remaining_assets.retain(|asset| asset.balance > 0.000001);

            recommendations.push(StrategyRecommendation {
                strategy: strategy.clone(),
                used_assets,
                vault_address: strategy.address.clone(),
            });
            ai_response_vec.push(ai_response);
        }


    }

    Json(CombinationResponse {
        ai_responses: ai_response_vec,
        recommendations,
    })
}


//pub async fn combination_pre<AI_: AI + Send + Sync + 'static >(
//    State(state): State<Arc<AppState<AI_>>>,
//    Json(payload): Json<UserInfo>,
//) -> impl IntoResponse {
//    let asset_names = extract_wallet_asset_names(&payload);
//    let risk_level = payload.risk;
//    let risk_use = risk_level.clone();
//    let stratigies = strategy_filter_by_depositable_asset( strategy_filter_by_risk(state.strategies.clone(), risk_level),asset_names );
//    let wallet_balances = payload.wallet_balance;
//
//
//    if let Some(assets) = wallet_balances {
//        let user_prompt = prompt_gen_combination(risk_use, assets, stratigies.clone()).await;
//        let ai_response = if let Ok(result) = state.ai_client.query(SYSTEM, user_prompt.as_str()).await {
//            result
//        } else {
//            "Wrong requests".to_string()
//        };
//        let index = hashtag_num_parser(&ai_response);
//
//        let chosen_stratigy = if let Some(st) = state.strategies.get(index).clone() {
//            st.clone()
//        } else {
//            state.strategies[0].clone()
//        };
//
//
//        Json(CombinationResponse_pre {
//            ai_responses: Some(ai_response),
//            strategies: Some(chosen_stratigy),
//        })
//    } else {
//        Json(CombinationResponse_pre {
//            ai_responses: None,
//            strategies: None,
//        })
//    }
//}
//

