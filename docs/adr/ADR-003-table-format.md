# ADR-004: Table Output Format

**Date**: 2026-04-02  
**Status**: Accepted  
**Deciders**: Agent  

## Context

heliosCLI displays tabular data (application lists, resource usage, etc.). The table formatting library choice impacts visual quality, performance, and feature availability.

## Decision

**Use tabled for all table output**

```rust
use tabled::{Table, Tabled, Style, Alignment, Modify};
use tabled::object::{Columns, Rows};

#[derive(Tabled)]
pub struct AppListItem {
    #[tabled(rename = "Name")]
    name: String,
    
    #[tabled(rename = "Status")]
    status: String,
    
    #[tabled(rename = "Backend")]
    backend: String,
    
    #[tabled(rename = "Uptime")]
    uptime: String,
    
    #[tabled(rename = "CPU")]
    cpu: String,
    
    #[tabled(rename = "Memory")]
    memory: String,
}

impl AppListItem {
    pub fn table(items: Vec<Self>) -> Table {
        Table::new(items)
            .with(Style::rounded())
            .with(Alignment::left())
            .modify(Columns::new(2..), Alignment::center())
    }
}
```

## Consequences

### Positive
- **Feature-rich**: Many styles, colors, alignment
- **Derive macro**: Easy struct-to-table
- **Active**: Well maintained

### Negative
- **Binary size**: ~100KB overhead

## References

- heliosCLI SOTA Research: `docs/research/CLI_FRAMEWORKS_TUI_SOTA.md`
