# Terminal-Bench 2.0 Optimization Plan

**Goal:** Beat Droid (69.9%) on Terminal-Bench 2.0  
**Current Codex Score:** ~62.9%

---

## 1. Understanding Terminal-Bench 2.0

### 1.1 What It Tests

- 89 diverse terminal tasks
- Unique environment per task
- Real-world workflows (debugging, refactoring, file manipulation)
- pytest-based verification (outcome-driven)

### 1.2 Scoring Metrics

- **Resolution Rate:** % of tasks completed successfully
- **Cost:** Token usage per task
- **Time:** Task completion time

### 1.3 Key Categories

1. File operations (grep, sed, awk)
2. Git operations
3. Package management
4. Debugging
5. Code modernization
6. Testing

---

## 2. Setup Terminal-Bench Framework

### 2.1 Install

```bash
# Install tbench CLI
uv tool install terminal-bench

# Verify
tb --help
```

### 2.2 Install Docker (required)

```bash
# macOS
brew install docker

# Start Docker Desktop
open -a Docker
```

### 2.3 Verify Setup

```bash
# List datasets
tb datasets list

# Run oracle (baseline)
tb run -d terminal-bench@2.0 -a oracle
```

---

## 3. Create Codex Adapter

### 3.1 Adapter Structure

```python
# adapters/codex.py
class CodexAdapter(BaseAgent):
    """Adapter for Codex CLI."""

    def __init__(self, model: str = "MiniMax-M2.5"):
        self.model = model
        self.binary = "/opt/homebrew/bin/codex"

    def run(self, task: Task) -> Result:
        # Execute codex with task prompt
        # Parse output
        # Return result
```

### 3.2 Key Implementation Details

- Map task prompts to codex commands
- Handle sandbox environment
- Parse verification output
- Capture metrics (time, tokens, errors)

---

## 4. Baseline Testing

### 4.1 Run Codex Baseline

```bash
tb run -d terminal-bench@2.0 \
  --agent codex \
  --model MiniMax-M2.5 \
  --output results/codex_baseline.json
```

### 4.2 Run Droid Baseline (for comparison)

```bash
tb run -d terminal-bench@2.0 \
  --agent droid \
  --output results/droid_baseline.json
```

### 4.3 Collect Metrics

- Resolution rate (%)
- Average time per task
- Token usage
- Error categories

---

## 5. Optimization Areas

### 5.1 Priority 1: Core Capabilities

| Area            | Current Issue      | Optimization                |
| --------------- | ------------------ | --------------------------- |
| Shell commands  | 90% timeout        | Increase timeout, add retry |
| Process control | Leaks on interrupt | Proper signal handling      |
| File operations | Inconsistent       | Better tool wrapping        |
| Git integration | Limited            | Native git support          |

### 5.2 Priority 2: Task-Specific

| Task Type   | Codex Issue      | Improvement             |
| ----------- | ---------------- | ----------------------- |
| Debugging   | Poor context     | Pre-read relevant files |
| Refactoring | Unsafe edits     | Add safety checks       |
| Testing     | Wrong assertions | Understand test intent  |
| Git         | Incomplete cmds  | Full git support        |

### 5.3 Priority 3: Meta-Improvements

| Area               | Current | Target             |
| ------------------ | ------- | ------------------ |
| Prompt engineering | Manual  | Optimized per task |
| Context window     | 192K    | Efficient usage    |
| Tool selection     | Fixed   | Dynamic            |
| Error recovery     | None    | Retry + fallback   |

---

## 6. Implementation Roadmap

### Phase 1: Foundation (Week 1-2)

- [ ] Install tbench framework
- [ ] Create Codex adapter
- [ ] Run baseline (expect ~62%)
- [ ] Run Droid baseline (~70%)

### Phase 2: Quick Wins (Week 3-4)

- [ ] Fix shell timeouts (configurable)
- [ ] Add process cleanup on interrupt
- [ ] Improve error messages
- [ ] Increase context efficiency

### Phase 3: Core Improvements (Week 5-8)

- [ ] Better tool orchestration
- [ ] Retry logic with backoff
- [ ] Dynamic tool selection
- [ ] Task-specific prompts

### Phase 4: Advanced (Week 9-12)

- [ ] Multi-agent coordination
- [ ] Self-correction loops
- [ ] Learning from failures
- [ ] Target: 75%+

---

## 7. Running Experiments

### 7.1 Test Specific Tasks

```bash
# Test single task
tb run -d terminal-bench@2.0 \
  --agent codex \
  --task-id hello-world
```

### 7.2 Test Category

```bash
# Test git tasks only
tb run -d terminal-bench@2.0 \
  --agent codex \
  --category git
```

### 7.3 Compare Models

```bash
# Test different models
tb run -d terminal-bench@2.0 --agent codex --model gpt-5.3-codex
tb run -d terminal-bench@2.0 --agent codex --model MiniMax-M2.5
tb run -d terminal-bench@2.0 --agent codex --model MiniMax-M2.5-highspeed
```

---

## 8. Success Criteria

| Metric       | Droid | Target | Stretch |
| ------------ | ----- | ------ | ------- |
| Resolution % | 69.9% | 72%    | 80%     |
| Avg Time     | -     | -20%   | -40%    |
| Cost/Task    | -     | -15%   | -30%    |

---

## 9. Notes

- Terminal-Bench requires Docker
- Some tasks may need specific environment setup
- Results vary by model choice
- Quality score matters (not just completion)

---

_Last Updated: 2026-02-23_
