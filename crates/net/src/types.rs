use std::env;
use std::error::Error;
use sonic_defai_defi::types::Strategy;
use sonic_defai_ai::ai::AI;
use sonic_defai_ai::claude::Claude;
use sonic_defai_ai::deepseek::DeepSeek;
use dotenv::dotenv;

pub struct AppState<AI_: AI> {
    pub strategies: Vec<Strategy>,
    pub ai_client: AI_,
}

impl AppState<Claude>{
    pub async fn new() -> Result<Self, dyn Error >{
        let file_content = tokio::fs::read_to_string("analyzed_vaults.json").await?;
        let v: Vec<Strategy> = serde_json::from_str(&file_content)?;
        dotenv().ok();
        let api_key = env::var("ANTHROPIC_API_KEY")
            .expect("ANTHROPIC_API_KEY must be set in .env file");

        Ok(AppState{
            strategies: v,
            ai_client: Claude::new("".to_string(), api_key)?
        })

    }
}


impl AppState<DeepSeek>{
    pub async fn new() -> Result<Self, dyn Error >{
        let file_content = tokio::fs::read_to_string("analyzed_vaults.json").await?;
        let v: Vec<Strategy> = serde_json::from_str(&file_content)?;

        let api_key = env::var("DEEPSEEK_API_KEY")
            .expect("DEEPSEEK_API_KEY must be set in .env file");

        Ok(AppState{
            strategies: v,
            ai_client: DeepSeek::new("".to_string(), api_key)?
        })

    }

}