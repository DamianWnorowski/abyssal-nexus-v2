use axum::{routing::post, Json, Router, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{MatrixX, MatrixNode, NodeType, FitnessScores, SealState, SealPolicy, evaluate_seal};
use chrono::Utc;

#[derive(Debug, Clone, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<Message>,
    pub session_id: Option<Uuid>,
    pub mode: Option<String>, // "UltraReason", "CostSaver", etc.
    pub max_depth: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatResponse {
    pub session_id: Uuid,
    pub content: String,
    pub reality_ids: Vec<Uuid>,
    pub root_node_id: Uuid,
    pub fitness: FitnessScores,
}

pub struct AppState {
    pub db: SqlitePool,
    pub matrix_x: Arc<Mutex<MatrixX>>,
    pub seal_policy: SealPolicy,
}

pub struct AbyssalConfig {
    pub max_depth: u32,
    pub base_branch_factor: u32,
    pub entropy_decay: f32,
}

impl Default for AbyssalConfig {
    fn default() -> Self {
        Self { max_depth: 6, base_branch_factor: 3, entropy_decay: 0.08 }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = SqlitePool::connect("sqlite://matrix_x.db").await.unwrap();
    sqlx::query!(
        r#"CREATE TABLE IF NOT EXISTS nodes (
            id TEXT PRIMARY KEY,
            session_id TEXT,
            node_type TEXT,
            parent_id TEXT,
            depth INTEGER,
            entropy_value REAL,
            fitness TEXT,
            seal TEXT,
            content TEXT,
            created_at TEXT
        )"#
    ).execute(&db).await.unwrap();

    let state = AppState {
        db,
        matrix_x: Arc::new(Mutex::new(MatrixX::new())),
        seal_policy: SealPolicy::default(),
    };

    let app = Router::new()
        .route("/api/ai-chat", post(ai_chat))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Abyssal Nexus v2 running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn ai_chat(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ChatRequest>,
) -> Json<ChatResponse> {
    let session_id = req.session_id.unwrap_or_else(Uuid::new_v4);
    
    let root_node = MatrixNode {
        id: Uuid::new_v4(),
        session_id,
        node_type: NodeType::Origin,
        parent_id: None,
        depth: 0,
        entropy_value: 0.8,
        fitness: FitnessScores::default(),
        provider_id: None,
        reality_id: Some(Uuid::new_v4()),
        seal: SealState::default(),
        content: None,
        created_at: Utc::now(),
    };

    let config = AbyssalConfig::default();
    let tree = generate_recursive_subtree(
        &state,
        &config,
        session_id,
        Some(root_node.clone()),
        0,
        0.8,
    ).await;

    persist_tree(&state.db, &tree).await;

    let content = "🌌 Abyssal Nexus v2 MVP: Recursive tree generated with Matrix X persistence and sealing.".to_string();

    Json(ChatResponse {
        session_id,
        content,
        reality_ids: vec![root_node.reality_id.unwrap()],
        root_node_id: root_node.id,
        fitness: FitnessScores {
            coherence: 0.95,
            hallucination_risk: 0.05,
            latency_ms: Some(2500),
            ..Default::default()
        },
    })
}

async fn generate_recursive_subtree(
    state: &AppState,
    config: &AbyssalConfig,
    session_id: Uuid,
    parent: Option<MatrixNode>,
    depth: u32,
    entropy: f32,
) -> Vec<MatrixNode> {
    if depth >= config.max_depth || entropy <= 0.0 {
        return vec![];
    }

    let mut result = vec![];
    let branch_factor = (config.base_branch_factor as f32 * entropy).ceil().max(1.0) as u32;

    for i in 0..branch_factor {
        let node = MatrixNode {
            id: Uuid::new_v4(),
            session_id,
            node_type: NodeType::Recursive,
            parent_id: parent.as_ref().map(|p| p.id),
            depth,
            entropy_value: entropy,
            fitness: FitnessScores::default(),
            provider_id: None,
            reality_id: parent.as_ref().and_then(|p| p.reality_id),
            seal: SealState::default(),
            content: None,
            created_at: Utc::now(),
        };

        let mut evaluated = evaluate_seal(node, &state.seal_policy);
        evaluated.content = Some(format!("Node d={} e={:.2} #{}", depth, entropy, i));

        result.push(evaluated);

        if !evaluated.seal.sealed {
            let next_entropy = (entropy - config.entropy_decay).max(0.0);
            let children = generate_recursive_subtree(state, config, session_id, 
                Some(evaluated.clone()), depth + 1, next_entropy).await;
            result.extend(children);
        }
    }
    result
}

async fn persist_tree(db: &SqlitePool, tree: &[MatrixNode]) {
    for node in tree {
        if let Err(e) = sqlx::query!(
            "INSERT OR REPLACE INTO nodes 
             (id, session_id, node_type, parent_id, depth, entropy_value, fitness, seal, content, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            node.id.to_string(),
            node.session_id.to_string(),
            format!("{:?}", node.node_type),
            node.parent_id.map(|id| id.to_string()),
            node.depth as i32,
            node.entropy_value,
            serde_json::to_string(&node.fitness).unwrap(),
            serde_json::to_string(&node.seal).unwrap(),
            node.content.as_deref(),
            node.created_at.to_rfc3339()
        ).execute(db).await {
            eprintln!("Failed to persist node {}: {}", node.id, e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    #[tokio::test]
    async fn test_sealing() {
        let mut node = MatrixNode {
            fitness: FitnessScores {
                hallucination_risk: 0.7,
                ..Default::default()
            },
            ..Default::default()
        };
        let sealed = evaluate_seal(node, &SealPolicy::default());
        assert!(sealed.seal.sealed);
    }
}