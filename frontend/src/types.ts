// Types
const Message = {
  role: string;
  content: string;
};

export interface ChatRequest {
  messages: Message[];
  session_id?: string;
  mode?: string;
  max_depth?: number;
}

export interface ChatResponse {
  session_id: string;
  content: string;
  reality_ids: string[];
  root_node_id: string;
  fitness: {
    coherence: number;
    hallucination_risk: number;
    latency_ms?: number;
  };
}

export type MatrixNodeType = 'Origin' | 'Meta' | 'Recursive' | 'Proxy' | 'Evaluator' | 'Output' | 'Mirage';

export interface MatrixNode {
  id: string;
  sessionId: string;
  nodeType: MatrixNodeType;
  parentId?: string;
  depth: number;
  entropyValue: number;
  fitness: {
    coherence: number;
    hallucination_risk: number;
  };
  seal: {
    sealed: boolean;
    reason?: string;
  };
}
