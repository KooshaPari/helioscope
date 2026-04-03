//! Benchmarks for harness_schema

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use harness_schema::{Command, Schema};

fn bench_schema_validate_valid(c: &mut Criterion) {
    let schema = Schema {
        name: "test_schema".to_string(),
        commands: vec![Command::new("cmd1", "echo 1"), Command::new("cmd2", "echo 2")],
    };
    c.bench_function("schema_validate_valid", |b| {
        b.iter(|| schema.validate());
    });
}

fn bench_schema_validate_empty_name(c: &mut Criterion) {
    let schema = Schema { name: "".to_string(), commands: vec![] };
    c.bench_function("schema_validate_empty_name", |b| {
        b.iter(|| schema.validate());
    });
}

fn bench_schema_command_count(c: &mut Criterion) {
    let schema = Schema {
        name: "test".to_string(),
        commands: vec![
            Command::new("a", "echo a"),
            Command::new("b", "echo b"),
            Command::new("c", "echo c"),
            Command::new("d", "echo d"),
            Command::new("e", "echo e"),
        ],
    };
    c.bench_function("schema_command_count", |b| {
        b.iter(|| schema.command_count());
    });
}

fn bench_schema_find_command_exists(c: &mut Criterion) {
    let schema = Schema {
        name: "test".to_string(),
        commands: vec![Command::new("findme", "echo found"), Command::new("other", "echo other")],
    };
    c.bench_function("schema_find_command_exists", |b| {
        b.iter(|| schema.find_command(black_box("findme")));
    });
}

fn bench_schema_find_command_missing(c: &mut Criterion) {
    let schema =
        Schema { name: "test".to_string(), commands: vec![Command::new("other", "echo other")] };
    c.bench_function("schema_find_command_missing", |b| {
        b.iter(|| schema.find_command(black_box("missing")));
    });
}

fn bench_command_new(c: &mut Criterion) {
    c.bench_function("command_new", |b| {
        b.iter(|| Command::new(black_box("test"), black_box("echo test")));
    });
}

fn bench_schema_new(c: &mut Criterion) {
    c.bench_function("schema_new", |b| {
        b.iter(|| Schema {
            name: black_box("test".to_string()),
            commands: black_box(vec![Command::new("a", "echo a"), Command::new("b", "echo b")]),
        });
    });
}

fn bench_schema_with_many_commands(c: &mut Criterion) {
    let commands: Vec<Command> =
        (0..100).map(|i| Command::new(&format!("cmd{}", i), &format!("echo {}", i))).collect();
    c.bench_function("schema_with_many_commands", |b| {
        b.iter(|| Schema {
            name: black_box("many_cmds".to_string()),
            commands: black_box(commands.clone()),
        });
    });
}

criterion_group!(
    benches,
    bench_schema_validate_valid,
    bench_schema_validate_empty_name,
    bench_schema_command_count,
    bench_schema_find_command_exists,
    bench_schema_find_command_missing,
    bench_command_new,
    bench_schema_new,
    bench_schema_with_many_commands
);
criterion_main!(benches);
