use std::error::Error;
use async_trait::async_trait;
use sonic_defai_defi::types::{Asset, Risk, Strategy};

pub struct AIError{
    pub(crate) msg: String
}

#[async_trait]
pub trait AI {
    fn new(model_name: String, api_key: String) -> Self;

    async fn query(&self, system: &str, input: &str) -> Result<String, AIError> ;

}



pub fn prompt_gen(risk_level: Risk, user_asset: Asset, filtered_stratigies: Vec<Strategy>) -> String{
    let risk: String = risk_level.into();
    let stratigies_description: Vec<String> = filtered_stratigies.into_iter().map(|s| s.into()).collect();
    let asset: String = user_asset.into();
    format!(
        "As a DeFi strategy advisor, provide a well-structured analysis with clear paragraphs \
        for a user seeking {} risk investments.\n
        user's Asset state: {} . You must think about strategies using this asset.\n
        Available Strategies:\n\n
        {}\n\n\
        **Please structure your response as follows**(**8Strictly follow the structure**):\n\
        1. Main Recommendation (2-3 paragraphs with clear line breaks)\n\
        2. Key Benefits (bullet points)\n\
        3. Risk Considerations\n\
        4. Strategy Comparisons (compare 2-3 strategies):\n\
           - Strategy_Name1 ##(numerical_metric)\n\
           - Strategy_Name2 ##(numerical_metric)\n\
           - Strategy_Name3 ##(numerical_metric)\n\n\
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