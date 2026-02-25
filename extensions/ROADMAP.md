# Extension Roadmap

## Phase 1: Foundation (Current)

- [x] Plugin protocol definition
- [x] Module contracts
- [ ] Base loader implementation
- [ ] Configuration schema

## Phase 2: Core Extensions

- [ ] CLI command extensions
- [ ] Handler system
- [ ] Adapter base

## Phase 3: Integration

- [ ] Codex CLI integration
- [ ] Harbor framework adapters
- [ ] Portage module system

## Phase 4: Ecosystem

- [ ] Extension registry
- [ ] Publishing tools
- [ ] Version compatibility

## Extension Points Summary

| Point | Type | Description |
|-------|------|-------------|
| command | CLI | Add new commands |
| adapter | Integration | Connect external systems |
| module | Core | Portage modules |
| renderer | Template | Custom templates |
| validator | Schema | Input/output validation |
| middleware | HTTP | Request/response processing |
| storage | Persistence | Custom backends |

## Migration Path

For existing code:
1. Identify extension point
2. Create plugin.yaml
3. Implement interface
4. Add tests
5. Publish to registry
