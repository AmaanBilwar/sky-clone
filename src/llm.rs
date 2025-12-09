use anyhow::anyhow;
use dotenv;
use rig::prelude::*;
use rig::{
    completion::Prompt,
    providers::gemini::{self},
};

pub fn send_to_llm(prompt: String) -> Result<String, anyhow::Error> {
    dotenv::dotenv().ok();
    // Initialize the Google Gemini client
    let client = gemini::Client::from_env();

    // Create agent with a single context prompt
    let agent = client
        .agent("gemini-2.5-flash")
        .preamble("Be creative and concise. Answer directly and clearly.")
        .temperature(0.1)
        .build();

    // Prompt the agent and return the response
    let response = agent.prompt(prompt);

    Ok(response)
}
