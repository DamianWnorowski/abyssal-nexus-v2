# Matrix X Schema

```sql
sqlite3 matrix_x.db ".schema nodes"

-- Query tree by session
SELECT * FROM nodes WHERE session_id = ? ORDER BY depth;

-- Sealed nodes
SELECT * FROM nodes WHERE json_extract(seal, '$.sealed') = 1;
```

## Test Request

```bash
curl -X POST http://localhost:3000/api/ai-chat \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"test"}],"session_id":null,"mode":"UltraReason"}'
```

Response includes `root_node_id` - query Matrix X for full tree.