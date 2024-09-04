# YonI Chat
* YonI Chat is a customizable desktop application built with Rust and Python to serve as a user-friendly interface for downloading and running text-generation large language models (LLMs) locally.
  
![cc5df25ffd713776edd27926f91ee479](https://github.com/user-attachments/assets/b0bcaf4b-a1a3-4a1c-a281-f2e6ff748ae1)


# Steps to use application
* Install Python 3.x, Rust, required crates (in `cargo.toml`).
* Set your python executable file in `cargo.toml`
* Optional: Modify python.rs to utilize any text-generation model available on HuggingFace `huggingface.co/models`
* Build the program with `cargo build`
* Run the program with `cargo run`
  
# Features
* Modern UI/UX
* Download text-generation LLM from HuggingFace
* Run the downloaded LLMs locally, providing fast response times and privacy.

# Features Planned
* Image generation support
* Personalized greeting from model
* User friendly scripting for download and run LLM's at runtime
  * LLM download progress
