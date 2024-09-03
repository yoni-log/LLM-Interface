# Rusty Interface
* Rusty Interface is a GUI wrapper to download and query locally run LLM's using HuggingFace, allowing the user to input any text-generation model to download, and query locally.
  [img]

# Steps to use application
* Requirements: Python 3.8+, Rust, crates (in cargo.toml)
* Set your python executable file in cargo.toml
* Optional: Modify python.rs to utilize any text-generation model available on HuggingFace `huggingface.co/models`
* Build the program with `cargo build`
* Run the program with `cargo run`
  
# Features
* Download any LLM from HuggingFace
* Run text-generation models

# Features Planned
* Image support
* User friendly scripting for download and run LLM's
