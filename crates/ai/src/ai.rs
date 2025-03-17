use anyhow::anyhow;
use async_trait::async_trait;
use sonic_defai_defi::types::{Asset, Risk, Strategy};

pub struct AIError{
    #[allow(dead_code)]
    pub(crate) msg: String
}

#[async_trait]
pub trait AI {
    fn new(_model_name: String, api_key: String) -> Self;

    async fn query(&self, system: &str, input: &str) -> Result<String, AIError> ;

}



pub fn prompt_gen(risk_level: Risk, user_asset: Asset, filtered_stratigies: Vec<Strategy>) -> String{
    let risk: String = risk_level.into();
    let stratigies_description: Vec<String> = filtered_stratigies.into_iter().map(|s| s.into()).collect();
    let asset: String = user_asset.into();
    format!(
        "As a DeFi strategy advisor, provide a well-structured analysis with clear paragraphs \
        for a user seeking {} risk investments.\n
        user's Asset state: {} . ***You must think about strategies using this asset. and u must pick one strategy in following stratigies***\n
        Available Strategies:\n\n
        {}\n\n\
        **Please structure your response as follows**(**Strictly follow the structure**):\n\
        1. Main Recommendation (2-3 paragraphs with clear line breaks) **U must start with 'I recommend  Strategy_Name #(Strategy index number)~**'\n\
        2. Key Benefits (bullet points)\n\
        3. Risk Considerations\n\
        4. Strategy Comparisons (compare 2-3 strategies):\n\
           - Strategy_Name1 #(Strategy index number)  ##(numerical_metric)\n\
           - Strategy_Name2 #(Strategy index number)  ##(numerical_metric)\n\
           - Strategy_Name3 #(Strategy index number)  ##(numerical_metric)\n\n\
        Focus on:\n\
        1. Clear paragraph structure\n\
        2. Risk-return analysis\n\
        3. Quantitative comparison\
        WARNING: When u struct the Comparsion Phase, you must attech #green or #red for each single setences for noticing which sentences must be displayed in Green for Red. Red texts mean which points of (Comparison target) are more worse then recommended strategy. Green texts mean Which points of(Comparison target) are better than recommended strategy.",
        risk,
        asset,
        stratigies_description.join("\n")
    )
}

pub async fn prompt_gen_combination(risk_level: Risk, user_assets: Vec<Asset>, filtered_stratigies: Vec<Strategy>) -> String{
    let risk: String = risk_level.into();
    let stratigies_description: Vec<String> = filtered_stratigies.into_iter().map(|s| s.into()).collect();

    let mut asset_descriptions = Vec::new();
    for asset in &user_assets {
        let price = fetch_asset_price(&asset.name).await;
        if let Ok(x) = price{
            let total_value = x * asset.balance;
            asset_descriptions.push(format!(
                "name: {}, in USD total value: ${:.2}",
                asset.name, total_value
            ));
        }
        else{
            asset_descriptions.push(
                "name: {}, SKip this Asset".to_string());
        }

    }

    format!(
        "As a DeFi strategy advisor, provide a well-structured analysis with clear paragraphs \
        for a user seeking {} risk investments.\n
        user's Asset state: {:?} . ***You must think about strategies using this asset combination - consider ALL possible number of cases (2^n-1)(explicitly reveal your thinking about number of cases ).\
        U must recommend optimal strategies for the each number of cases, then u finally recommend optimal combinations of one or several (Asset case, Strategy) - For example, for asset A,B,C,D -> u can return (A,B,strategy #13) (C,strategy #45) (D,strategy #65)    \
        Don't just look at surface-level factors, but deeply analyze based on the user's asset amounts as well. - U must explicitly reveal your thinking process at Main Recommendation phase. *** \
        And u can also refer to SWAP. I mean, swap, u can recommend user SWAP some assets to other assets first, then conduct strategy.** \n
        Available Strategies:\n\n
        {}\n\n\
        **Please structure your response as follows**(**Strictly follow the structure**):\n\
        1. Main Recommendation (4-5 paragraphs with clear line breaks) **WARNING: U must start with 'I recommend following combinations,  Strategy_Name #(Strategy index number)~ for (assets) and, Strategy_Name #(Strategy index number)~ for (another assets). . .   **'\n\
        2. Key Benefits (bullet points)\n\
        3. Risk Considerations\n\
        4. Strategy Comparisons (compare 2-3 strategies):\n\
           - Strategy_Name1 #(Strategy index number)  ##(numerical_metric)\n\
           - Strategy_Name2 #(Strategy index number)  ##(numerical_metric)\n\
           - Strategy_Name3 #(Strategy index number)  ##(numerical_metric)\n\n\
        5. Expected return(numerically, also considering user asset amount)
        Focus on:\n\
        1. Clear paragraph structure\n\
        2. Risk-return analysis\n\
        3. Quantitative comparison\
        WARNING: When u struct the Comparsion Phase, you must attech #green or #red for each single setences for noticing which sentences must be displayed in Green for Red. Red texts mean which points of (Comparison target) are more worse then recommended strategy. Green texts mean Which points of(Comparison target) are better than recommended strategy.",
        risk,
        asset_descriptions,
        stratigies_description.join("\n")
    )
}




use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct PriceResponse {
    usd: f64,
}


async fn fetch_asset_price(mut asset_name: &str) -> Result<f64, anyhow::Error> {
    let client = Client::new();

    if asset_name.to_lowercase() == "sonic"{
        asset_name = "sonic-3"
    }

    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        asset_name.to_lowercase()
    );

    // 에러 처리를 anyhow로 단순화
    let resp = client.get(&url)
        .send()
        .await?;

    let json: Value = resp.json().await?;

    // 안전하게 JSON에서 값 추출
    json.get(asset_name.to_lowercase())
        .and_then(|coin| coin.get("usd"))
        .and_then(|price|  price.as_f64())
        .ok_or_else(|| anyhow!("Price not found for {}", asset_name))
}
