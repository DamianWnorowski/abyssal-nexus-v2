import React, { useState, useCallback, useEffect } from 'react';
import ReactFlow, { Background, Controls, MiniMap, useNodesState, useEdgesState } from 'reactflow';
import 'reactflow/dist/style.css';
import axios from 'axios';
import { v4 as uuidv4 } from 'uuid';
import { openDB } from 'idb';

type MatrixNode = {
  id: string;
  sessionId: string;
  nodeType: string;
  parentId?: string;
  depth: number;
  entropyValue: number;
  fitness: any;
  seal: any;
};

type Edge = {
  id: string;
  source: string;
  target: string;
};

const initialNodes = [];
const initialEdges: Edge[] = [];

const AbyssalWorkbench: React.FC = () => {
  const [nodes, setNodes] = useNodesState(initialNodes);
  const [edges, setEdges] = useEdgesState(initialEdges);
  const [sessionId, setSessionId] = useState<string>('');
  const [status, setStatus] = useState<string>('');

  const loadMatrixX = useCallback(async () => {
    try {
      const db = await openDB('matrix_x', 1, {
        upgrade(db) {
          const store = db.createObjectStore('nodes', { keyPath: 'id' });
          store.createIndex('session_id', 'session_id');
        },
      });
      const allNodes = await db.getAll('nodes');
      const sessionNodes = allNodes.filter((n: MatrixNode) => n.sessionId === sessionId);

      const flowNodes = sessionNodes.map((node: MatrixNode) => ({
        id: node.id,
        type: node.seal.sealed ? 'input' : 'default',
        data: { 
          label: `${node.nodeType} (e:${node.entropyValue.toFixed(2)}) ${node.seal.sealed ? '[SEALED]' : ''}`,
          fitness: node.fitness,
        },
        position: { x: node.depth * 250, y: Math.random() * 400 + (node.id.charCodeAt(0) % 10) * 40 },
        style: node.seal.sealed ? { background: '#ff4444', color: 'white' } : undefined,
      }));

      const flowEdges: Edge[] = [];
      for (const node of sessionNodes) {
        if (node.parentId) {
          flowEdges.push({
            id: `${node.parentId}-${node.id}`,
            source: node.parentId,
            target: node.id,
          });
        }
      }

      setNodes(flowNodes);
      setEdges(flowEdges);
      setStatus(`Loaded ${sessionNodes.length} nodes from Matrix X`);
    } catch (error) {
      console.error('Failed to load Matrix X:', error);
      setStatus('Failed to load IndexedDB Matrix X');
    }
  }, [sessionId, setNodes, setEdges]);

  const generateTree = useCallback(async () => {
    try {
      const response = await axios.post('/api/ai-chat', {
        messages: [{ role: 'user', content: `Generate ABYSSAL tree for session ${uuidv4()}` }],
        mode: 'UltraReason',
      });
      
      setSessionId(response.data.session_id);
      setStatus(`Generated tree ${response.data.root_node_id}`);
      setTimeout(loadMatrixX, 1000);
    } catch (error) {
      console.error('Failed to generate tree:', error);
      setStatus('Failed to reach backend');
    }
  }, [loadMatrixX]);

  useEffect(() => {
    if (sessionId) {
      loadMatrixX();
    }
  }, [sessionId, loadMatrixX]);

  return (
    <div style={{ height: '100vh', width: '100vw', display: 'flex', flexDirection: 'column' }}>
      <div style={{ 
        padding: '1rem', 
        background: '#1a1a1a', 
        color: 'white', 
        display: 'flex', 
        gap: '1rem',
        borderBottom: '1px solid #333'
      }}>
        <h1 style={{ margin: 0 }}>🌌 Abyssal Workbench</h1>
        <button 
          onClick={generateTree}
          style={{ padding: '0.5rem 1rem', background: '#00aa88', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}
        >
          Generate Tree
        </button>
        <button 
          onClick={loadMatrixX}
          disabled={!sessionId}
          style={{ padding: '0.5rem 1rem', background: '#666', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}
        >
          Load Session: {sessionId.slice(0,8)}
        </button>
        <span style={{ marginLeft: 'auto' }}>{status}</span>
      </div>
      <div style={{ flex: 1 }}>
        <ReactFlow nodes={nodes} edges={edges}>
          <Background />
          <Controls />
          <MiniMap />
        </ReactFlow>
      </div>
    </div>
  );
};

export default AbyssalWorkbench;
