use anthropic::client::Client;
use anthropic::client::ClientBuilder;
use anthropic::types::{ContentBlock, MessagesRequestBuilder};
use async_trait::async_trait;
use crate::ai::{AIError, AI};

pub struct Claude{
    client: Client
}

#[async_trait]
impl AI for Claude{
    fn new(_model_name: String, api_key: String) ->  Self {
        let client = ClientBuilder::default()
            .api_key(api_key)
            .build().unwrap();
        Claude{
            client
        }

    }

    async fn query(&self, _system: &str, input: &str) -> Result<String, AIError> {
        let message_request = MessagesRequestBuilder::default()
            .model("claude-3-7-sonnet-20250219".to_string())
            .temperature(0)
            .max_tokens(9000_usize)
            .system("You are a DeFi investment expert. Provide structured responses with clear paragraphs and numerical comparisons in detail ( Give Long-mathematical-Logical Investment strategy Report), .  Especially, **When u struct the Comparsion Phase, you must attech #green or #red for each single setences for noticing which sentences must be displayed in Green for Red. Red texts mean which points of (Comparison target) are more worse then recommended strategy. Green texts mean Which points of(Comparison target) are better than recommended strategy. **".to_string())
            .messages(vec![
                anthropic::types::Message {
                    role: anthropic::types::Role::User,
                    content: vec![ContentBlock::Text { text: input.to_string() }],
                },
            ])
            .build();

        if let Ok(result) = message_request {
            let message_response = self.client.messages(result).await;

            if let Ok(msg) = message_response{
                let recommendation = msg.content.iter()
                    .filter_map(|block| {
                        if let anthropic::types::ContentBlock::Text { text } = block {
                            Some(text.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("");
                Ok(recommendation)
            }
            else{
                Err(AIError{msg: "Fail to fetch msg response".to_string()})
            }
        }
        else{
            Err(AIError{msg: "Fail to build msg request".to_string()})
        }
    }
}
