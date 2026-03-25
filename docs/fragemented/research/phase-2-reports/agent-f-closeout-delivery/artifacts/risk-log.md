# Lane F Risk Log

- R1: `goose`/`kilocode` lockfile parity checks were planned but not fully executed due non-deterministic environment execution.
- R2: Opencode and cliproxy strictness remains WARN due partial evidence and archive/status signals.
- R3: Full command execution for phase-2 strict profile was intentionally bounded to avoid long-running test churn.
- R4: Validation runner script is implemented as scaffold; deterministic idempotence and retry semantics are pending full command-suite execution.
