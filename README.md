# YonI Chat
* YonI Chat is a customizable desktop application built with Rust and Python to serve as a user-friendly interface for downloading and running text-generation large language models (LLMs) locally.
  
![image](https://github.com/user-attachments/assets/cbb825d3-ad26-41f0-83e9-7b84f261e43d)

# Installation & setup
1. Clone Repository
* `git clone https://github.com/yoni-log/LLM-Interface.git`
* `cd LLM-Interface`
3. Install Rependencies
* Rust: Download and install Rust from [rust-lang.org](https://www.rust-lang.org/).
* Python 3.x: Download and install Python from [python.org](https://www.python.org/).
* Required Python Libraries:
  * `pip install transformers`
3. Configure Python Path
* Set your `python.exe` file in `cargo.toml`
4. Build and Run the Program
* Build the program with `cargo build`
* Run the program with `cargo run`

# Features
* Modern User Interface: YonI Chat features a responsive UI built with the egui framework.
* Local LLM Execution: Download and run text-generation models locally, ensuring total data privacy.
* Model Selection: Easily input model tags (e.g., "openai-community/gpt2") to download and run different models.
* Text-based Conversation: Engage in seamless text-based conversations with the selected LLMs.
* Model Exploration: Direct links to Hugging Face’s model library for easy model exploration and selection.

# Features Planned
* Image Generation Support: Expand functionality to include image generation models from Hugging Face.
* Model Download Progress: Display real-time progress during model downloads for better user feedback.
* Dynamic Responses: Customize your name and the app's greeting for a more personalized user experience.
* Advanced Scripting: Provide users with scripting capabilities to handle LLM downloads and executions at runtime.
