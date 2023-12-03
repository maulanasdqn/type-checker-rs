
use reqwest;
use std::env;
use std::fs;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: program <file1> <file2>");
        return Ok(());
    }

    let file1_contents = fs::read_to_string(&args[1])?;
    let file2_contents = fs::read_to_string(&args[2])?;

    let api_key = env::var("CHATGPT_API_KEY").expect("CHATGPT_API_KEY not set");

    let client = reqwest::Client::new();
    let response = client.post("https://api.openai.com/v1/engines/chatgpt/completions")
        .bearer_auth(api_key)
        .json(&serde_json::json!({
            "prompt": format!("{}\n{}", file1_contents, file2_contents),
            "max_tokens": 200
        }))
        .send()
        .await?;

    let response_text = response.text().await?;
    println!("Response from ChatGPT: {}", response_text);

    Ok(())
}

