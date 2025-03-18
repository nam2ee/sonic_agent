use async_trait::async_trait;
use deepseek_rs::DeepSeekClient;
use deepseek_rs::request::{MaxTokens, Message, Model, RequestBody, Temperature};
use crate::ai::{AIError, AI};

pub struct DeepSeek{
    client: DeepSeekClient
}

#[async_trait]
impl AI for DeepSeek {
    fn new(_model_name: String, api_key: String) -> Self {
        let client = DeepSeekClient::new_with_api_key(api_key);
        DeepSeek{
            client
        }
    }

    async fn query(&self, system: &str, input: &str) -> Result<String, AIError> {
        let client = &self.client;
        let request = RequestBody::new_messages(vec![Message::new_system_message(system.to_string()),Message::new_user_message(input.to_string())])
            .with_max_tokens(MaxTokens::new(7000))
            .with_temperature(Temperature::new(0_f32))
            .with_model(Model::DeepSeekReasoner);

        let response = client.chat_completions(request).await;

        if let Ok(result ) = response{
            let output = result.choices[0].message.content.clone();
            match output {
                Some(s) =>{
                    Ok(s)
                }
                None => {
                    Ok("Failed".to_string())
                }
            }
        }
        else{
            Err(AIError{msg:"Request Failed".to_string()})
        }
    }
}