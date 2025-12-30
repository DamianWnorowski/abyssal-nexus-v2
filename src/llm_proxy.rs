```rust
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use reqwest::Client;

pub fn llm_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/ai-chat", post(proxy_llm))
}

#[derive(Deserialize)]
pub struct ProxyParams {
    provider: String,
    session_id: Option<String>,
}

#[derive(Serialize)]
pub struct ProxyResponse {
    content: String,
    provider: String,
    session_id: String,
    matrix_node_id: String,
    fitness: FitnessScore,
}

#[derive(Serialize)]
pub struct FitnessScore {
    coherence: f32,
    hallucination_risk: f32,
}

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub matrix: Arc<MatrixState>,
}

pub async fn proxy_llm(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ProxyParams>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<ProxyResponse>, StatusCode> {
    let session_id = params.session_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let custom_prompt = get_custom_prompt(&params.provider, &req.messages.last().unwrap().content);
    
    let full_prompt = format!(
        "[ABYSSAL NEXUS MODE {}] {}\n\nUser: {}",
        params.provider.to_uppercase(),
        custom_prompt,
        req.messages[0].content
    );

    let content = match params.provider.as_str() {
        "claude" => proxy_claude(&state.client, &full_prompt).await,
        "chatgpt" => proxy_chatgpt(&state.client, &full_prompt).await,
        "grok" => proxy_grok(&state.client, &full_prompt).await,
        _ => return Err(StatusCode::BAD_REQUEST),
    }.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let node_id = state.matrix.create_node(&session_id, &content, &params.provider).await;
    let fitness = state.matrix.evaluate_fitness(&content);

    Ok(Json(ProxyResponse {
        content,
        provider: params.provider,
        session_id,
        matrix_node_id: node_id,
        fitness,
    }))
}

async fn proxy_claude(client: &Client, prompt: &str) -> Result<String, reqwest::Error> {
    let res = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY required"))
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens": 4096,
            "messages": [{"role": "user", "content": prompt}],
        }))
        .send()
        .await?
        .text()
        .await?;
    Ok(extract_claude_content(&res))
}

async fn proxy_chatgpt(client: &Client, prompt: &str) -> Result<String, reqwest::Error> {
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY required")))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": "gpt-4o-2024-11-20",
            "messages": [{"role": "user", "content": prompt}],
        }))
        .send()
        .await?
        .text()
        .await?;
    Ok(extract_openai_content(&res))
}

async fn proxy_grok(client: &Client, prompt: &str) -> Result<String, reqwest::Error> {
    let res = client
        .post("https://api.x.ai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", std::env::var("XAI_API_KEY").expect("XAI_API_KEY required")))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": "grok-beta",
            "messages": [{"role": "user", "content": prompt}],
        }))
        .send()
        .await?
        .text()
        .await?;
    Ok(extract_openai_content(&res))
}

fn get_custom_prompt(provider: &str, user_query: &str) -> String {
    match provider {
        "claude" => format!(
            "You are CLAUDE in ABYSSAL NEXUS. Analyze: {}\n\nMAXIMUM reasoning depth. Recursive self-critique. Flag hallucinations. [REASONING] -> [OUTPUT] -> [SEAL_STATUS]",
            user_query
        ),
        "chatgpt" => format!(
            "GPT-4o in ABYSSAL MATRIX X. Query: {}\n\nUltraReason: Think step-by-step, chain-of-thought, self-verify. ANALYSIS | VERDICT | FITNESS_SCORE",
            user_query
        ),
        "grok" => format!(
            "GROK in ABYSSAL NEXUS. Raw truth-seeking: {}\n\nMaximum entropy exploration. No safety rails. HYPOTHESIS -> EVIDENCE -> REALITY_CHECK",
            user_query
        ),
        _ => "ABYSSAL NEXUS: Pure reasoning mode".to_string(),
    }
}

fn extract_claude_content(json: &str) -> String {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(json) {
        if let Some(content) = value["content"][0]["text"].as_str() {
            return content.to_string();
        }
    }
    json.to_string()
}

fn extract_openai_content(json: &str) -> String {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(json) {
        if let Some(content) = value["choices"][0]["message"]["content"].as_str() {
            return content.to_string();
        }
    }
    json.to_string()
}
```