# Data Model: Codex TUI Renderer Optimization Research

## Entities

### RendererStage
- id: string
- name: string
- category: enum(input, schedule, compose, draw, flush)
- owner_module: string
- measurable_outputs: list[string]

### BackendStage
- id: string
- name: string
- category: enum(entrypoint, thread, model_client, protocol, ui_dispatch)
- owner_module: string
- measurable_outputs: list[string]

### EventClass
- id: string
- source: string
- priority: enum(critical, normal, low)
- burst_profile: enum(low, medium, high)
- coalescible: boolean

### OptimizationCandidate
- id: string
- title: string
- layer: string
- expected_impact: enum(low, medium, high)
- risk_level: enum(low, medium, high)
- dependencies: list[string]
- evidence_refs: list[string]

### LayeredPR
- id: string
- name: string
- order_index: integer
- objective: string
- success_signal: string
- depends_on: list[string]

### EvidenceRecord
- evidence_id: string
- source_id: string
- claim: string
- confidence: enum(low, medium, high)

## Relationships
- RendererStage emits EventClass.
- BackendStage emits EventClass.
- OptimizationCandidate targets one or more RendererStage or BackendStage entries.
- LayeredPR implements one or more OptimizationCandidate entries.
- EvidenceRecord supports OptimizationCandidate and LayeredPR rationale.
