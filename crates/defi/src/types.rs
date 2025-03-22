use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize, Clone, PartialEq)]
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

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct Asset{
    pub name: String,
    pub balance: f64
}

impl From<Asset> for String{
    fn from(asset: Asset) -> Self {
        format!("{}: {}", asset.name , asset.balance)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub wallet_balance: Option<Vec<Asset>>,
    pub risk: Risk
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldRates {
    pub income: Income,
    pub vs_hold: Vec<VsHold>,
    pub prices: Vec<Price>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Income {
    #[serde(rename = "total apy")]
    pub total_apy: RateValues,
    #[serde(rename = "total apr")]
    pub total_apr: RateValues,
    #[serde(rename = "pool swap fees apr", default)]
    pub pool_swap_fees_apr: RateValues,
    #[serde(rename = "farm apr")]
    pub farm_apr: RateValues,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateValues {
    pub latest: String,
    #[serde(rename = "24h")]
    pub day: String,
    pub week: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VsHold {
    #[serde(rename = "type")]
    pub hold_type: String,
    #[serde(rename = "apr_24h")]
    pub apr_day: String,
    pub apr_week: String,
    pub est_apr: String,
    #[serde(rename = "days_57")]
    pub days_57: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub token: String,
    pub init_price: String,
    pub price: String,
    #[serde(rename = "change_57_days")]
    pub change_57_days: String,
}

impl From<YieldRates> for String {
    fn from(rates: YieldRates) -> Self {
        let mut result = String::new();

        // Income section
        result.push_str("Income:\n");
        result.push_str(&format!("  Total APY: Latest: {}, 24h: {}, Week: {}\n",
                                 rates.income.total_apy.latest,
                                 rates.income.total_apy.day,
                                 rates.income.total_apy.week));

        result.push_str(&format!("  Total APR: Latest: {}, 24h: {}, Week: {}\n",
                                 rates.income.total_apr.latest,
                                 rates.income.total_apr.day,
                                 rates.income.total_apr.week));

        // Pool Swap Fees APR - 이제 Option이 아니라 default 값을 사용하므로 조건 확인
        if !rates.income.pool_swap_fees_apr.latest.is_empty() ||
            !rates.income.pool_swap_fees_apr.day.is_empty() ||
            !rates.income.pool_swap_fees_apr.week.is_empty() {
            result.push_str(&format!("  Pool Swap Fees APR: Latest: {}, 24h: {}, Week: {}\n",
                                     rates.income.pool_swap_fees_apr.latest,
                                     rates.income.pool_swap_fees_apr.day,
                                     rates.income.pool_swap_fees_apr.week));
        }

        result.push_str(&format!("  Farm APR: Latest: {}, 24h: {}, Week: {}\n",
                                 rates.income.farm_apr.latest,
                                 rates.income.farm_apr.day,
                                 rates.income.farm_apr.week));

        // VS Hold section
        result.push_str("\nVS Hold Performance:\n");
        for hold in &rates.vs_hold {
            // "-" 값인 경우 "N/A"로 표시
            let apr_day = if hold.apr_day == "-" { "N/A".to_string() } else { hold.apr_day.clone() };
            let apr_week = if hold.apr_week == "-" { "N/A".to_string() } else { hold.apr_week.clone() };
            let est_apr = if hold.est_apr == "-" { "N/A".to_string() } else { hold.est_apr.clone() };
            let days_57 = if hold.days_57 == "-" { "N/A".to_string() } else { hold.days_57.clone() };

            result.push_str(&format!("  {}: 24h: {}, Week: {}, Est APR: {}, 57 Days: {}\n",
                                     hold.hold_type,
                                     apr_day,
                                     apr_week,
                                     est_apr,
                                     days_57));
        }

        // Prices section
        result.push_str("\nPrice Changes:\n");
        for price in &rates.prices {
            result.push_str(&format!("  {}: Initial: {}, Current: {}, Change (57d): {}\n",
                                     price.token,
                                     price.init_price,
                                     price.price,
                                     price.change_57_days));
        }

        result
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Strategy {
    pub(crate) name: String,
    pub(crate) risk_level: Risk,
    pub(crate) risk_reason: String,
    pub(crate) description: String,
    pub(crate) impermanent_loss: Option<String>,
    pub(crate) impermanent_loss_description: Option<String>,
    pub(crate) yield_rates: YieldRates,
    pub(crate) depositable_asset: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) original_index: Option<usize>,
}


impl From<Strategy> for String{
    fn from(strategy: Strategy) -> Self {
        let risk: String = strategy.risk_level.into();
        let yield_rates: String = strategy.yield_rates.into();

        let il = if let Some(x) = strategy.impermanent_loss {
             x
        }
        else{ "N/A".to_string()};

        let ild = if let Some(y) = strategy.impermanent_loss_description {
            y
        }
        else{ "N/A".to_string()};

        let idx = if let Some(z) = strategy.original_index {
            z
        }
        else{
            0
        };
        format!(
            "Strategy index number #{}:\nName: {} Description: {}\n Usable asset for this strategy: {:?} Risk Level: {} Reason for risk level: {}\n Impermanent loss: {} impermanent_loss_description: {} \n YieldRates: {} ",
            idx,
            strategy.name,
            strategy.description,
            strategy.depositable_asset,
            risk,
            strategy.risk_reason,
            il,
            ild,
            yield_rates
        )
    }
}

