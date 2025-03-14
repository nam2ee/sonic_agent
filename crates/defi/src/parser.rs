use std::cmp::PartialEq;
use crate::types::{Asset, Risk, Strategy};

pub fn strategy_filter(v: Vec<Strategy>, risk: Risk) -> Vec<Strategy>{
    let mut result = vec![];
    let _= match risk{
        Risk::High => {
            for (idx, strategy) in v.iter().enumerate() {
                if strategy.risk_level == Risk::High {
                    result.push(strategy.clone());
                }
            }
        }
        Risk::Medium=>{
            for (idx, strategy) in v.iter().enumerate() {
                if strategy.risk_level == Risk::Medium {
                    result.push(strategy.clone());
                }
            }
        }
        Risk::Low=>{
            for (idx, strategy) in v.iter().enumerate() {
                if strategy.risk_level == Risk::Low {
                    result.push(strategy.clone());
                }
            }
        }
        Risk::Others=>{
                for (idx, strategy) in v.iter().enumerate() {
                if strategy.risk_level == Risk::Low {
                result.push(strategy.clone());
                }
            }
        }
    };
    result
}