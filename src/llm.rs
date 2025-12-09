use dotenv;
use rig::agent::stream_to_stdout;
use rig::prelude::*;
use rig::providers::gemini::completion::gemini_api_types::{
    AdditionalParameters, GenerationConfig, ThinkingConfig,
};
use rig::{
    providers::gemini::{self},
    streaming::StreamingPrompt,
};

pub async fn send_to_llm(prompt: String) -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let gen_cfg = GenerationConfig {
        thinking_config: Some(ThinkingConfig {
            include_thoughts: Some(false),
            thinking_budget: 2048,
        }),
        ..Default::default()
    };
    let cfg = AdditionalParameters::default().with_config(gen_cfg);
    // Create streaming agent with a single context prompt
    let agent = gemini::Client::from_env()
        .agent(gemini::completion::GEMINI_2_0_FLASH)
        .preamble("Be precise and concise.")
        .temperature(0.5)
        .additional_params(serde_json::to_value(cfg).unwrap())
        .build();

    // Stream the response and print chunks as they arrive
    let mut stream = agent.stream_prompt(prompt).await;

    let res = stream_to_stdout(&mut stream).await?;

    println!("Final text response: {message:?}", message = res.response());

    Ok(())
}
