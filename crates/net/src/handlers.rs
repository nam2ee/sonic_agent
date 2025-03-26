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
use crate::parser::{hashtag_num_parser, parse_asset_info};
use crate::types::{AppState, CombinationResponse, CombinationResponse_pre, RecommendationResponse, StrategyRecommendation};

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

pub async fn combination_pre<AI_: AI + Send + Sync + 'static >(
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


        Json(CombinationResponse_pre {
            ai_responses: Some(ai_response),
            strategies: Some(chosen_stratigy),
        })
    } else {
        Json(CombinationResponse_pre {
            ai_responses: None,
            strategies: None,
        })
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

    let filtered_strategies = strategy_filter_by_depositable_asset(
        strategy_filter_by_risk(state.strategies.clone(), risk_level.clone()),
        asset_names
    );


    while !remaining_assets.is_empty() && !filtered_strategies.is_empty() {
        // AI 추천 생성
        let user_prompt = prompt_gen_combination(
            risk_level.clone(),
            remaining_assets.clone(),
            filtered_strategies.clone()
        ).await;

        let ai_response = match state.ai_client.query(SYSTEM, user_prompt.as_str()).await {
            Ok(result) => result,
            Err(_) => break,
        };

        // AI 응답 파싱
        let strategy_index = hashtag_num_parser(&ai_response);
        let asset_info = parse_asset_info(&ai_response);


        if let Some(strategy) = state.strategies.get(strategy_index).cloned() {

            let mut used_assets = Vec::new();
            for (asset_name, amount_str) in asset_info {
                println!("{} ,{}", asset_name,amount_str);
                if let Ok(amount) = amount_str.parse::<f64>() {
                    // remaining_assets에서 사용된 자산 차감
                    if let Some(pos) = remaining_assets.iter().position(|a| a.name == asset_name) {
                        if remaining_assets[pos].balance >= amount {
                            remaining_assets[pos].balance -= amount;
                            used_assets.push((asset_name, amount));
                        }
                    }
                }
            }

            remaining_assets.retain(|asset| asset.balance > 0.000001);


            recommendations.push(StrategyRecommendation {
                strategy: strategy.clone(),
                used_assets,
                vault_address: strategy.address.clone(),
            });
            ai_response_vec.push(ai_response);
        }

        // 최소 자산 임계값 확인 또는 최대 전략 수 도달 확인
        if recommendations.len() >= 4 || remaining_assets.iter().all(|a| a.balance < 0.000001) {
            break;
        }
    }

    Json(CombinationResponse {
        ai_responses: ai_response_vec,
        recommendations,
    })
}

