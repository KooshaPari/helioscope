//! Benchmarks for harness_orchestrator

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use harness_orchestrator::{
    Agent, AgentCapability, AgentStatus, RootManager, Task, TaskPriority, TaskStatus,
};
use uuid::Uuid;

fn bench_id_new(c: &mut Criterion) {
    c.bench_function("id_new", |b| {
        b.iter(|| Uuid::new_v4());
    });
}

fn bench_task_new(c: &mut Criterion) {
    c.bench_function("task_new", |b| {
        b.iter(|| {
            Task::new(black_box("spec-1"), black_box("task-name"), black_box("task-description"))
        });
    });
}

fn bench_task_short_id(c: &mut Criterion) {
    let task = Task::new("spec", "name", "desc");
    c.bench_function("task_short_id", |b| {
        b.iter(|| task.short_id());
    });
}

fn bench_task_is_ready_true(c: &mut Criterion) {
    let task = Task::new("spec", "name", "desc");
    let completed = vec![task.id];
    c.bench_function("task_is_ready_true", |b| {
        b.iter(|| task.is_ready(black_box(&completed)));
    });
}

fn bench_task_is_ready_false(c: &mut Criterion) {
    let task = Task::new("spec", "name", "desc");
    let completed = vec![];
    c.bench_function("task_is_ready_false", |b| {
        b.iter(|| task.is_ready(black_box(&completed)));
    });
}

fn bench_agent_new(c: &mut Criterion) {
    c.bench_function("agent_new", |b| {
        b.iter(|| Agent::new(black_box("agent-1"), black_box(vec![AgentCapability::General])));
    });
}

fn bench_agent_is_available(c: &mut Criterion) {
    let agent = Agent::new("agent", vec![AgentCapability::General]);
    c.bench_function("agent_is_available", |b| {
        b.iter(|| agent.is_available());
    });
}

fn bench_agent_assign_release(c: &mut Criterion) {
    let mut agent = Agent::new("agent", vec![AgentCapability::General]);
    let task_id = Uuid::new_v4();
    c.bench_function("agent_assign_release", |b| {
        b.iter(|| {
            agent.assign(black_box(task_id));
            agent.release(black_box(true));
        });
    });
}

fn bench_root_manager_new(c: &mut Criterion) {
    c.bench_function("root_manager_new", |b| {
        b.iter(|| RootManager::new());
    });
}

fn bench_task_status_transitions(c: &mut Criterion) {
    let mut task = Task::new("spec", "name", "desc");
    let agent_id = Uuid::new_v4();
    c.bench_function("task_status_transitions", |b| {
        b.iter(|| {
            task.start(black_box(agent_id));
            task.complete(black_box("success"));
        });
    });
}

fn bench_task_priority_ordering(c: &mut Criterion) {
    c.bench_function("task_priority_ordering", |b| {
        b.iter(|| {
            let _ = TaskPriority::Critical < TaskPriority::Low;
            let _ = TaskPriority::High < TaskPriority::Normal;
        });
    });
}

fn bench_agent_capability_check(c: &mut Criterion) {
    let caps = vec![AgentCapability::CodeGen, AgentCapability::Testing, AgentCapability::General];
    c.bench_function("agent_capability_iterate", |b| {
        b.iter(|| {
            for cap in &caps {
                let _ = format!("{:?}", cap);
            }
        });
    });
}

criterion_group!(
    benches,
    bench_id_new,
    bench_task_new,
    bench_task_short_id,
    bench_task_is_ready_true,
    bench_task_is_ready_false,
    bench_agent_new,
    bench_agent_is_available,
    bench_agent_assign_release,
    bench_root_manager_new,
    bench_task_status_transitions,
    bench_task_priority_ordering,
    bench_agent_capability_check
);
criterion_main!(benches);
