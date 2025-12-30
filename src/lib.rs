use petgraph::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Origin, Meta, Recursive, Proxy, Evaluator, Output, Mirage
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FitnessScores {
    pub coherence: f32,
    pub hallucination_risk: f32,
    pub latency_ms: Option<u64>,
    pub cost_tokens: Option<u64>,
    pub entropy: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SealState {
    pub sealed: bool,
    pub reason: Option<String>,
    pub triggered_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixNode {
    pub id: Uuid,
    pub session_id: Uuid,
    pub node_type: NodeType,
    pub parent_id: Option<Uuid>,
    pub depth: u32,
    pub entropy_value: f32,
    pub fitness: FitnessScores,
    pub provider_id: Option<Uuid>,
    pub reality_id: Option<Uuid>,
    pub seal: SealState,
    pub content: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub type MatrixX = StableDiGraph<MatrixNode, f32>; // edges = fitness weights

pub struct SealPolicy {
    pub max_hallucination_risk: f32,
    pub min_coherence: f32,
    pub max_latency_ms: Option<u64>,
}

impl Default for SealPolicy {
    fn default() -> Self {
        Self {
            max_hallucination_risk: 0.6,
            min_coherence: 0.3,
            max_latency_ms: Some(10000),
        }
    }
}

pub fn evaluate_seal(mut node: MatrixNode, policy: &SealPolicy) -> MatrixNode {
    if node.seal.sealed { return node; }
    
    let mut reasons = Vec::new();
    let fit = &node.fitness;
    
    if fit.hallucination_risk > policy.max_hallucination_risk {
        reasons.push(format!("hallucination_risk: {:.2} > {:.2}", 
            fit.hallucination_risk, policy.max_hallucination_risk));
    }
    if fit.coherence < policy.min_coherence {
        reasons.push(format!("coherence: {:.2} < {:.2}", 
            fit.coherence, policy.min_coherence));
    }
    if let (Some(max_lat), Some(lat)) = (policy.max_latency_ms, fit.latency_ms) {
        if lat > max_lat {
            reasons.push(format!("latency_ms: {} > {}", lat, max_lat));
        }
    }
    
    if !reasons.is_empty() {
        node.seal = SealState {
            sealed: true,
            reason: Some(reasons.join(", ")),
            triggered_at: Some(Utc::now()),
        };
    }
    node
}