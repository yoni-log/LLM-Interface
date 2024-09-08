use egui::TextBuffer;
use crate::python;

pub struct Conversations {
    pub conversations: Vec<(String, String)>,
    pub model: String,
}

impl Conversations  {
    pub fn new() -> Self {
        Self {
            conversations: Vec::new(),
            model: "Input your model here!".to_string(),
        }
    }

    pub fn push_conversation(&mut self, user_input: &str, gpt_output: &str) {
        self.conversations.push((user_input.to_string(), gpt_output.to_string()));
    }

    pub fn get_last_output(&self) -> String {
        self.conversations
            .last()
            .map(|(_, gpt_output)| gpt_output.clone())
            .unwrap_or_else(|| "No conversation yet".to_string())
    }

    pub fn send_input(conversations: &mut Conversations, input: &str) -> bool {
        let input_and_output = Self::get_output(conversations.model.as_str(), input);
        let gpt_output = input_and_output.split_at(input.len() + 1).1;
        conversations.push_conversation(input, gpt_output);
        true
    }

    pub fn get_output(model: &str, input: &str) -> String {
        match python::get_llm_output(input.as_str(), model.as_str()) {
            Ok(out) => out.to_string(),
            Err(E) => format!("{input}-Error: {E}").to_string()
        }
    }
}
