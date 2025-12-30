**🚀 Multi-LLM Proxy Deployed**

**New endpoint:** `/api/ai-chat?provider=claude|chatgpt|grok`

**Frontend update:** Change `generateTree` to:
```tsx
const response = await axios.post('/api/ai-chat?provider=claude', {
  messages: [{ role: 'user', content: `Generate ABYSSAL tree for session ${uuidv4()}` }],
  mode: 'UltraReason',
});
```

**Env vars (docker/.env):**
```
ANTHROPIC_API_KEY=sk-ant-...
OPENAI_API_KEY=sk-proj-...
XAI_API_KEY=xai-...
DATABASE_URL=sqlite://matrix_x.db
```

**Test:**
```bash
curl "http://localhost:3000/api/ai-chat?provider=claude" \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"test abyssal reasoning"}],"mode":"UltraReason"}'
```

**Custom Prompts:**
- **Claude:** Recursive self-critique `[REASONING] -> [OUTPUT] -> [SEAL_STATUS]`
- **ChatGPT:** UltraReason `ANALYSIS | VERDICT | FITNESS_SCORE`
- **Grok:** Raw truth-seeking `HYPOTHESIS -> EVIDENCE -> REALITY_CHECK`

**Matrix X:** Auto-saves responses w/ fitness scores [file:116]