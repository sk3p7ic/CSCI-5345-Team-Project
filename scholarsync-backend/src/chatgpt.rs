use serde_json::{Value, json};

const DESC_PROMPT: &str = "Please generate a summary of research interests for
this professor from their list of paper titles. If a description cannot be
determined, please return a dummy description. Descriptions should be at most
256 characters.";

pub struct ModelRequest(pub String);

impl ModelRequest {
    pub async fn make(&self) -> Result<String, String> {
        let client = async_openai::Client::new();
        let res: Value = client
            .chat()
            .create_byot(json!({
                "messages": [
                    {
                        "role": "developer",
                        "content": DESC_PROMPT
                    },
                    {
                        "role": "user",
                        "content": self.0
                    }
                ],
                "model": "gpt-4o-mini",
                "store": false
            }))
            .await
            .map_err(|e| format!("Error making request to ChatGPT: {e}"))?;
        Ok(res["choices"][0]["message"]["content"]
            .to_string()
            .trim_matches('"')
            .to_string())
    }
}
