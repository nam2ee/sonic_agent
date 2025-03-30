use crate::types::{Risk, Strategy, UserInfo};

pub fn strategy_filter_by_risk(v: Vec<Strategy>, risk: Risk) -> Vec<Strategy>{
    let mut result = vec![];
    let _= match risk{
        Risk::High => {
            for (_idx, strategy) in v.iter().enumerate() {
                if strategy.risk_level == Risk::High {
                    result.push(strategy.clone());
                }
                else if strategy.risk_level == Risk::Medium {
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
    let user_assets: Vec<String> = asset.into_iter().map(|s| s.to_lowercase()).collect();

    v.into_iter()
        .filter(|strategy| {
            strategy.depositable_asset.iter().any(|depositable| {
                let depositable_lower = depositable.to_lowercase();

                // LP 풀 형식인지 확인 (LP(...) 형태)
                if depositable_lower.starts_with("lp(") && depositable_lower.ends_with(")") {
                    // LP(...) 내부의 자산들을 추출
                    let inner = depositable_lower
                        .trim_start_matches("lp(")
                        .trim_end_matches(")")
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<String>>();

                    // LP 풀의 모든 자산이 사용자 자산 목록에 있는지 확인
                    inner.iter().all(|asset| user_assets.contains(asset))
                } else {
                    // 단일 자산인 경우, 사용자 자산 목록에 있는지 확인
                    user_assets.contains(&depositable_lower)
                }
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
