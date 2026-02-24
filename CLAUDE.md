# helios_router_data — Claude Instructions

This project uses Streamlit for the UI. Key files:

- `app.py` - Main Streamlit application
- `src/helios_router_ui/db/schema.py` - Database layer
- `src/helios_router_ui/pareto/engine.py` - Pareto computation
- `src/helios_router_ui/ui/components.py` - UI components

## Commands

```bash
task dev          # Start Streamlit
task lint         # Run ruff
task test        # Run pytest
task check       # All checks
```

## Notes

- Uses `uv` for package management
- Database is SQLite (`helios.db`)
- Optional NATS integration for real-time events
