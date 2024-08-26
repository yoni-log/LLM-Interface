pub struct Conversations {
    pub conversations: Vec<(String, String)>,
}

impl Conversations  {
    pub fn new() -> Self {
        Self {
            conversations: Vec::new(),
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
        // temporary debug example
        let gpt_output = "Washington D.C.".to_string();
        // let gpt_output = receive_output(); // in the future let receive_output call the python file
        conversations.push_conversation(&input, &gpt_output);
        true
    }

    pub fn get_output() -> String {
        "Washington D.C.".to_string()
    }
}
