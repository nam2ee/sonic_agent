use crate::types::{Risk, Strategy, UserInfo};

pub fn strategy_filter_by_risk(v: Vec<Strategy>, risk: Risk) -> Vec<Strategy>{
    let mut result = vec![];
    let _= match risk{
        Risk::High => {
            for (_idx, strategy) in v.iter().enumerate() {
                if strategy.risk_level == Risk::High {
                    result.push(strategy.clone());
                }
            }
        }
        Risk::Medium=>{
            for (_idx, strategy) in v.iter().enumerate() {
                if strategy.risk_level == Risk::Medium {
                    result.push(strategy.clone());
                }
            }
        }
        Risk::Low=>{
            for (_idx, strategy) in v.iter().enumerate() {
                if strategy.risk_level == Risk::Low {
                    result.push(strategy.clone());
                }
            }
        }
        Risk::Others=>{
                for (_idx, strategy) in v.iter().enumerate() {
                if strategy.risk_level == Risk::Low {
                result.push(strategy.clone());
                }
            }
        }
    };
    result
}


pub fn strategy_filter_by_depositable_asset(v: Vec<Strategy>, asset: Vec<String>) -> Vec<Strategy> {
    // 전달받은 asset 리스트를 모두 lowercase로 변환
    let target_assets: Vec<String> = asset.into_iter().map(|s| s.to_lowercase()).collect();

    v.into_iter()
        .filter(|strategy| {
            // 각 전략의 depositable_asset의 항목을 lowercase로 변환해서 target_assets에 포함되는지 확인
            strategy.depositable_asset.iter().any(|a| {
                target_assets.contains(&a.to_lowercase())
            })
        })
        .collect()
}


pub fn extract_wallet_asset_names(user_info: &UserInfo) -> Vec<String> {
    user_info.wallet_balance.as_ref()
        .map(|assets| {
            assets.iter().map(|asset| asset.name.to_lowercase()).collect()
        })
        .unwrap_or_default()
}
