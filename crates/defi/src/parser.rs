use crate::types::{Risk, Strategy};

pub fn strategy_filter(v: Vec<Strategy>, risk: Risk) -> Vec<Strategy>{
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