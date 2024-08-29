use hf_hub::api::sync::Api;
use tokenizers::tokenizer::{Tokenizer, Result};

pub fn get_tokens() -> Result<(Vec<String>)> {
    // Download config from model and get the file path
    let api = Api::new().unwrap();
    let repo = api.model("openai-community/gpt2".to_string());
    let _file_name = repo.get("config.json").unwrap();
    println!("{}", _file_name.display());

    // Encode return generated tokens from text input
    let tokenizer = Tokenizer::from_pretrained("openai-community/gpt2", None)?;
    let encode = tokenizer.encode("Once upon a time, ", false)?;
    let gen_tokens = encode.get_tokens().to_vec();
    println!("{:?}", encode.get_tokens().to_vec());
    Ok((gen_tokens))
}
