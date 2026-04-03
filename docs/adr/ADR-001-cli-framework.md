# ADR-003: CLI Framework Selection

**Date**: 2026-04-02  
**Status**: Accepted  
**Deciders**: Agent  

## Context

heliosCLI requires a command-line interface framework for parsing arguments, subcommands, and generating help text. The choice impacts developer experience, compile times, binary size, and feature availability.

## Decision Drivers

- **Ecosystem**: Community size, documentation quality
- **Compile time**: Debug and release build speed
- **Binary size**: CLI overhead
- **Features**: Shell completions, man pages, validation
- **Async support**: Integration with async runtime
- **Maintainability**: Long-term viability

## Options Considered

### Option A: clap (Derive API)

**GitHub**: [clap-rs/clap](https://github.com/clap-rs/clap)  
**Stars**: 14k+ | **Downloads**: 200M+

**Pros**:
- De facto standard (95%+ of Rust CLIs)
- Rich features: completions, man pages, colors
- Derive macro API is ergonomic
- Active development (v4 in 2023)
- Excellent documentation and examples
- Validation and error messages built-in

**Cons**:
- Larger binary size (~300KB) than minimal options
- Slower compile than argh/bpaf
- Feature bloat if minimal CLI needed

**Performance**:
- Compile: ~3-5s (debug), ~15-30s (release)
- Binary: +200-400KB
- Runtime: <1ms parse

### Option B: argh

**GitHub**: [google/argh](https://github.com/google/argh)  
- Google-maintained
- Minimal, opinionated
- Very fast compile
- Tiny binaries (~50KB overhead)

**Cons**:
- No shell completions
- No man page generation
- Minimal features
- Smaller ecosystem

**Verdict**: Good for embedded/resource-constrained, but insufficient for heliosCLI's needs

### Option C: bpaf

**GitHub**: [pacak/bpaf](https://github.com/pacak/bpaf)
- Compositional design
- Fast compile
- Good error messages
- Smaller than clap

**Cons**:
- Smaller ecosystem
- Fewer built-in features
- Less documentation

**Verdict**: Good alternative, but clap's ecosystem dominance is decisive

## Decision

**Adopt clap with derive API as the CLI framework**.

### Implementation

```rust
use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(name = "helios")]
#[command(about = "Helioscope application manager")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    /// Backend to use (docker, kubernetes, local, sandbox)
    #[arg(short, long, global = true, env = "HELIOS_BACKEND")]
    pub backend: Option<String>,
    
    /// Configuration file path
    #[arg(short, long, global = true, env = "HELIOS_CONFIG")]
    pub config: Option<PathBuf>,
    
    /// Verbose output (use multiple times for more verbosity)
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,
    
    /// Disable colored output
    #[arg(long, global = true, env = "NO_COLOR")]
    pub no_color: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Deploy an application
    Deploy(DeployArgs),
    
    /// List running applications
    List(ListArgs),
    
    /// Show application logs
    Logs(LogsArgs),
    
    /// Execute a command in running application
    Exec(ExecArgs),
    
    /// Build container image
    Build(BuildArgs),
    
    /// Manage configuration
    #[command(subcommand)]
    Config(ConfigCommands),
}

#[derive(Args)]
pub struct DeployArgs {
    /// Application name
    pub name: String,
    
    /// Deployment target environment
    #[arg(short, long, default_value = "local")]
    pub target: String,
    
    /// Docker image to deploy
    #[arg(short, long)]
    pub image: Option<String>,
    
    /// Environment variables (KEY=VALUE)
    #[arg(short, long = "env", value_parser = parse_key_value)]
    pub env_vars: Vec<(String, String)>,
    
    /// Port mappings (HOST:CONTAINER)
    #[arg(short, long = "port", value_parser = parse_port_mapping)]
    pub ports: Vec<(u16, u16)>,
    
    /// Perform a dry run without deploying
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Args)]
pub struct ListArgs {
    /// Show all applications (including stopped)
    #[arg(short, long)]
    pub all: bool,
    
    /// Output format
    #[arg(short, long, value_enum, default_value = "table")]
    pub format: OutputFormat,
    
    /// Filter by backend
    #[arg(short, long)]
    pub backend: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
}
```

### Feature Configuration

```toml
[dependencies.clap]
version = "4.5"
features = [
    "derive",       # Derive macro API
    "env",          # Environment variable support
    "cargo",        # Cargo integration (version)
    "unicode",      # Unicode support
    "wrap_help",    # Help text wrapping
    "color",        # Colored output
]
```

### Generated Artifacts

**Shell completions**:
```bash
# Generate completion scripts
helios completions bash > /etc/bash_completion.d/helios
helios completions zsh > /usr/share/zsh/site-functions/_helios
helios completions fish > ~/.config/fish/completions/helios.fish
```

**Man pages**:
```bash
# Generate man pages
helios man > /usr/share/man/man1/helios.1
```

## Consequences

### Positive
- **Industry standard**: Best practices built-in
- **Rich features**: Completions, man pages, validation
- **Active ecosystem**: Many examples and plugins
- **Long-term viability**: Backed by rust-cli working group

### Negative
- **Binary size**: +200-400KB overhead
- **Compile time**: Slower than minimal options
- **Complexity**: Many features (learning curve)

### Neutral
- **Performance**: Negligible runtime overhead (<1ms)

## Alternatives Not Chosen

- **argh**: Good for embedded, lacks features needed
- **bpaf**: Interesting design, ecosystem too small
- **structopt**: Deprecated (merged into clap v3)

## References

- clap: https://github.com/clap-rs/clap
- clap book: https://docs.rs/clap/latest/clap/
- Command Line Interface Guidelines: https://clig.dev/
- heliosCLI SOTA Research: `docs/research/CLI_FRAMEWORKS_TUI_SOTA.md`

---

*This ADR will be updated as implementation progresses*
