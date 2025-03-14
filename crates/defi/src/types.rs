use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Risk{
    Low,
    Medium,
    High,
    #[serde(other)]
    Others // error for unknown risk type!
}

impl From<Risk> for String{
    fn from(risk: Risk) -> Self {
        match risk{
            Risk::High => {
                String::from("high")
            }
            Risk::Medium => {
                String::from("medium")
            }
            Risk::Low => {
                String::from("low")
            }
            Risk::Others => {
                String::from("medium")
            }
        }

    }
}

#[derive(Debug,Deserialize, Clone)]
pub struct Asset{
    name: String,
    balance: u32
}

impl From<Asset> for String{
    fn from(asset: Asset) -> Self {
        format!("{}: {}", asset.name , asset.balance)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserInfo {
    pub wallet_balance: Option<Vec<Asset>>,
    pub risk: Risk
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Strategy {
    pub(crate) name: String,
    pub(crate) risk_level: Risk,
    pub(crate) description: String,
    pub(crate) risk_reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) original_index: Option<usize>,
}


impl From<Strategy> for String{
    fn from(strategy: Strategy) -> Self {
        let risk: String = strategy.risk_level.into();
        format!(
            "Strategy #{}:\nName: {}\nRisk Level: {}\nDescription: {}\n Reason for risk level: {}",
            strategy.original_index.unwrap(),
            strategy.name,
            risk,
            strategy.description,
            strategy.risk_reason,
        )
    }
}

