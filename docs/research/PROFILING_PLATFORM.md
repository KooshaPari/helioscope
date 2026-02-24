# Platform Profiling Guide

## Platform Profiling Tools

### macOS

| Tool | Purpose | Command |
|------|----------|----------|
| Instruments | CPU/Memory | open -n Instruments.app |
| Time Profiler | Hot paths | Instruments |
| DTrace | System tracing | dtrace -n 'syscall::write:entry' |
| leaks | Memory | leaks binary |

### Linux

| Tool | Purpose | Command |
|------|----------|----------|
| perf | CPU profiling | perf record -g |
| BCC/eBPF | Kernel tracing | profile-bpfcc |
| flamegraph | Visualization | flamegraph.pl |
| bpftrace | Scripts | bpftrace -e 'kprobe:do_nanosleep' |

### Windows

| Tool | Purpose |
|------|----------|
| ETW | Events |
| WPR | Recording |
| GPU | GPU profiling |

## Shell Optimization

### Profiling Tools

| Tool | Platform | Use |
|------|----------|-------|
| hyperfine | Cross-platform | Benchmarking |
| timep | bash/zsh | Stack traces |
| zprof | ZSH | Init time |
| sh | POSIX | Profiling |

### Best Practices

1. **Minimize external commands** - Use built-ins (45% faster)
2. **Profile first** - hyperfine
3. **Lazy load plugins** - Speed up startup
4. **Batch operations** - Reduce forks
5. **Native loops** - Avoid subshells

## References

- Instruments: developer.apple.com
- perf: man7.org/man
- eBPF: github.com/iovisor/bcc
- hyperfine: github.com/sharkdp/hyperfine
