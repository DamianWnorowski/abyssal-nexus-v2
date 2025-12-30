```rust
// Matrix X state (stub for proxy integration)
use petgraph::Graph;
use uuid::Uuid;
use sqlx::{SqlitePool, Row};
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct MatrixState {
    pub pool: SqlitePool,
    pub graph: Arc<RwLock<Graph<String, f32>>>,
}

#[derive(Serialize, Deserialize)]
pub struct FitnessScore {
    pub coherence: f32,
    pub hallucination_risk: f32,
}

impl MatrixState {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect("sqlite://matrix_x.db").await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS nodes (
            id TEXT PRIMARY KEY,
            session_id TEXT,
            content TEXT,
            provider TEXT,
            depth INTEGER,
            entropy_value REAL,
            seal JSON
        )").execute(&pool).await?;
        Ok(Self {
            pool,
            graph: Arc::new(RwLock::new(Graph::new())),
        })
    }

    pub async fn create_node(&self, session_id: &str, content: &str, provider: &str) -> String {
        let node_id = Uuid::new_v4().to_string();
        let depth = 0; // Simplified
        let entropy_value = content.len() as f32 / 1000.0;
        
        sqlx::query("INSERT INTO nodes (id, session_id, content, provider, depth, entropy_value, seal) 
                    VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(&node_id)
            .bind(session_id)
            .bind(content)
            .bind(provider)
            .bind(depth)
            .bind(entropy_value)
            .bind(serde_json::json!({"sealed": false}).to_string())
            .execute(&self.pool)
            .await
            .unwrap();
        
        node_id
    }

    pub fn evaluate_fitness(&self, _content: &str) -> FitnessScore {
        FitnessScore {
            coherence: 0.85,
            hallucination_risk: 0.12,
        }
    }
}
```

**Dependencies in Cargo.toml:**
```toml
[dependencies]
axum = "0.7"
reqwest = { version = "0.12", features = ["json"] }
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "sqlite"] }
petgraph = "0.6"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4", "serde"] }
tokio::sync = "RwLock"
```