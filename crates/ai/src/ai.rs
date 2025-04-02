use anyhow::{anyhow, Error};
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



pub async fn prompt_gen(risk_level: Risk, user_asset: Asset, filtered_stratigies: Vec<Strategy>) -> String {
    let risk: String = risk_level.into();
    let stratigies_description: Vec<String> = filtered_stratigies.into_iter().map(|s| s.into()).collect();

    let mut asset_description = "".to_string();


        let price = fetch_asset_price(&user_asset.name).await;
        if let Ok(x) = price {
            let total_value = x * user_asset.balance;
            asset_description = format!(
                "name: {}, balance: {}, in USD total value: ${:.2}",
                user_asset.name, user_asset.balance, total_value
            );
        } else {
            asset_description = format!(
                "name: {}, balance: {}",
                user_asset.name, user_asset.balance,
            );
        }

    format!(
        "As a DeFi strategy advisor, provide a well-structured analysis with clear paragraphs \
        for a user seeking {} risk investments.\n
        user's Asset state: {:?} - using this - not convert allow to another assets. ***You must think about strategies **actually using this asset** - (actually usable asset for each strategies are denoted in depositable_asset field)**. and u must pick one optimal strategy in following stratigies***\n
        Available Strategies:\n\n
        {}\n\n\
        **Please structure your response as follows**(**Strictly follow the structure**):\n\
        1. Main Recommendation (2-3 paragraphs with clear line breaks) U must strictly start with <I recommend #(Strategy index number) Strategy - (Strategy_Name) and corresponding depositable_assets are [(depositable_asset)] from them, I can pick asset which is also included in user asset state; #Asset_name: (asset_name) #Asset_balance: (asset_balance for conducting strategy- in decimal(not percentage) - Allocate the MAXIMUM POSSIBLE AMOUNT(100%)  from the user's balance)>\n\
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
        asset_description,
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
                "name: {}, balance: {}, in USD total value: ${:.2}",
                asset.name, asset.balance, total_value
            ));
        }
        else{
            asset_descriptions.push(
                "name: {}, SKip this Asset".to_string());
        }

    }

//U must return usable asset in both user assets and strategy. for example, if depositable_asset are [LP(wS,TAILS), wS, USDC, stS], u must pick one in it and it also usable in user balance.

    format!(
        "As a DeFi strategy advisor, provide a well-structured analysis with clear paragraphs \
        for a user seeking {} risk investments.\n
        user's Asset state: {:?} . ***You must think about strategies using this asset combination - consider ALL possible number of cases (2^n-1)(explicitly reveal your thinking about number of cases ).\
        ***Don't just look at surface-level factors, but deeply analyze based on the user's asset amounts as well. - U must explicitly reveal your thinking process at Main Recommendation phase. *** \

        think in the way\n\
        0. seek which asset(s) from user asset state is optimal choice for using as deposit asset.
        1. U must recommend only one **user-asset based available** strategy considering user asset state. - I mean, **user-asset based available**, user has at least, one asset of strategy's depositable_asset.

        Available Strategies:\n\
        {}\n\n\
        Reference: in depositable_asset, LP(A,B) is Liquidity pool deposit. So, if u choose LP, u must ensure both asset A and B are in user balance. -> if usable, u can return Asset#1 and Asset#2 ***.
        **Please structure your response as follows**(**Strictly follow the structure**) !we are in production so u must ensure following Main Recommendation's rule! :\n\
        1. Main Recommendation (4-5 paragraphs with clear line breaks - strictly write ensuring - about(starting format, asset_amount rule) following WARNING)  \
        ***WARNING: PLZ Don't recommend zero Asset#_amount: 0. u must Allocate the MAXIMUM POSSIBLE AMOUNT from the user's balanc ;***
        **WARNING: Case1. IF u chose Single asset for strategy, U must strictly start with <I recommend following combination,  #(Strategy index number) Strategy - (Strategy_Name) and corresponding depositable_assets are [(depositable_asset)] from them, I can pick asset(s) which is also included in user asset state; Asset#1_name: (asset1_name) Asset#1_balance: (asset1_balance for conducting strategy- in decimal - Allocate the MAXIMUM POSSIBLE AMOUNT(100%)  from the user's balance)> \
        Case2. Dont be afraid for recommending LP pool deposit. If u chose LP asset pair for strategy, U must strictly start with <I recommend following combination,   #(Strategy index number) Strategy - (Strategy_Name) and corresponding depositable_assets are [(depositable_asset)] from them, I can pick asset(s) which is also included in user asset state; Asset#1_name: (asset1_name) Asset#1_balance: (asset1_balance for conducting strategy in decimal - Allocate the MAXIMUM POSSIBLE AMOUNT(100%) from the user's balance) and Amount#2_name: (asset2_name) Asset#2_balance: (asset2_balance for conducting strategy in decimal- Allocate the MAXIMUM POSSIBLE AMOUNT(100%)  from the user's balance)>  **'\n\
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
        WARNING: When u struct the Comparsion Phase, you must attech #green or #red for each single setences for noticing which sentences must be displayed in Green for Red. Red texts mean which points of (Comparison target) are more worse then recommended strategy. Green texts mean Which points of(Comparison target) are better than recommended strategy.
        *** ENSURE: So, the most important thing in your recommendation is ensuring Asset_1 or Asset_1, Asset_2 pair is real in user's Asset state.***.
        **When recommend  Asset_1, Asset_2 pair (actually, LP pool deposit), U must ensure the both assets in user asset state.***. ",
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


async fn fetch_asset_price(mut asset_name: &str) -> Result<f64, Error> {
    let client = Client::new();

    // Map asset ticker to its CoinGecko API name.
    // For assets not explicitly mapped, the input asset_name will be used.
    let mapped_asset = match asset_name.to_lowercase().as_str() {
        "sonic" => "sonic-3",
        "ws" => "wrapped-sonic",
        "sacra" => "sacra",
        "swpx" => "swapx-2",
        "ateth" => "atoll-eth",
        "sceth" => "rings-sceth",
        "os" => "origin-staked-s",
        "wos" => "wrapped-sonic-origin",
        "usdc.e" => "sonic-bridged-usdc-e-sonic",
        "scusd" => "rings-scusd",
        "equal" => "equalizer-on-sonic",
        "tails" => "tails-2",
        "brush" => "paintswap",
        "fsonic" => "fantomsonicinu-2",
        "sdog" => asset_name,  // no mapping provided
        "moon" => "moon-bay",
        "donks" => asset_name, // no mapping provided
        "eco" => "fantom-eco-2",   // no mapping provided
        "sts" | "st s" => "beets-staked-sonic",
        "xshadow" => "shadow-2",
        "indi" => "indi",  // no mapping provided
        "fs" => "fantomstarter",    // no mapping provided
        "weth" => "weth",
        "stbl" => "stability",
        "tysg" => asset_name,  // no mapping provided
        "goglz" => "goggles",
        "missor" => asset_name,  // no mapping provided
        "sacra_gem_1" => asset_name,  // no mapping provided
        "fbomb" => "fbomb", // no mapping provided
        "eggs" => "eggs-finance",
        "navi" => "navigator-exchange",
        "beets" => "beethoven-x",
        _ => asset_name
    };

    let asset_name = asset_name.to_lowercase();

    if asset_name == "donks"{
        return Ok(0.00002109_f64)
    }
    else if asset_name == "tysg"{
        return Ok(0.0004813_f64)
    }
    else if asset_name =="missor"{
        return Ok(0.03461_f64)
    }
    else if asset_name =="eco"{
        return Ok(0.25_f64)
    }
    else if asset_name=="sdog"{
        return Ok(0.01346_f64)
    }
    else if asset_name == "sacra_gem_1"{
        return Ok(0.4241_f64)
    }
    else if asset_name == "usdc"{
        return Ok(1_f64)
    }
    else if asset_name == "ws"{
        return Ok(0.5_f64)
    }
    else if asset_name == "os"{
        return Ok(0.5_f64)
    }
    else if asset_name == "wos" {
        return Ok(0.5_f64)
    }
    else if asset_name == "sts" {
        return Ok(0.5_f64)
    }
    else if asset_name == "beets"{
        return Ok(0.04_f64)
    }
    else if asset_name == "ateth"{
        return Ok(2100_f64)
    }
    else if asset_name == "sceth"{
        return Ok(2100_f64)
    }
    else if asset_name == "sonic"{
        return Ok(0.4900_f64)
    }
    else if asset_name == "sacra"{
        return Ok(0.01410_f64)
    }
    else if asset_name == "swpx"{
        return Ok(0.2650_f64)
    }
    else if asset_name == "usdc.e"{
        return Ok(1.0000_f64)
    }
    else if asset_name == "scusd"{
        return Ok(0.9983_f64)
    }
    else if asset_name == "equal"{
        return Ok(1.8400_f64)
    }
    else if asset_name == "tails"{
        return Ok(0.02694_f64)
    }
    else if asset_name == "brush"{
        return Ok(0.01859_f64)
    }
    else if asset_name == "fsonic"{
        return Ok(0.001757_f64)
    }
    else if asset_name == "moon"{
        return Ok(0.002280_f64)
    }
    else if asset_name == "xshadow"{
        return Ok(53.9500_f64)
    }
    else if asset_name == "indi"{
        return Ok(0.01474_f64)
    }
    else if asset_name == "fs"{
        return Ok(0.0001957_f64)
    }
    else if asset_name == "weth"{
        return Ok(1810.5000_f64)
    }
    else if asset_name == "stbl"{
        return Ok(0.07211_f64)
    }
    else if asset_name == "goglz"{
        return Ok(0.1044_f64)
    }
    else if asset_name == "fbomb"{
        return Ok(0.02405_f64)
    }
    else if asset_name == "eggs"{
        return Ok(0.0005501_f64)
    }
    else if asset_name == "navi"{
        return Ok(1.1100_f64)
    }


    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        mapped_asset
    );

    // Send the GET request.
    let resp = client.get(&url)
        .send()
        .await?;

    // Parse JSON response.
    let json: Value = resp.json().await?;

    // Safely extract the price from the JSON response.
    json.get(mapped_asset)
        .and_then(|coin| coin.get("usd"))
        .and_then(|price| { price.as_f64()})
        .ok_or_else(|| anyhow!("Price not found for {}", mapped_asset))
}
