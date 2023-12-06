use tokio;
use std::fs;
use std::env;
use chatgpt::prelude::*;

type Type = std::result::Result<(), Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Type {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: program <file1> <file2>");
        return Ok(());
    }
    let file1_contents = fs::read_to_string(&args[1])?;
    let file2_contents = fs::read_to_string(&args[2])?;

    let prompt = r#"You are a type checker that help user to check whether the given type definition is compatible with given API Contract.
                    The user will give several type definitions and corresponding API Contract. You should give list of answers.
                Example:
                - There are 4 cases and each case consist of TYPE & API Contract
                
                Goal:
                - Compare the type & API Contract in each case
                - Show the result of each case in a JSON array format.
                - Give the reason if the API Contract is not compatible with the type.
                
                Expected Output Format:
                [
                  {
                    "result": true,
                  },
                  {
                    "result": false,
                    "reason": "attribute `x` should be integer."
                  },
                  {
                    "result": true,
                  },
                  {
                    "result": true,
                  }
                ]
                
                WARNING: please response with the exactly format as described above without any additional information."#;

    let result = &[prompt, &file1_contents, &file2_contents].concat();

    let key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY");
    let client = ChatGPT::new(key)?;
    let response = client
        .send_message(result)
        .await?;
    println!("Response: {}", response.message().content);
    Ok(())
}
