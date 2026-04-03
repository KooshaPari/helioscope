//! Benchmarks for harness_queue

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use harness_queue::{Channel, RingBuffer, WorkQueue};

fn bench_channel_new(c: &mut Criterion) {
    c.bench_function("channel_new", |b| {
        b.iter(|| Channel::<u64>::new(black_box(1000)));
    });
}

fn bench_channel_send(c: &mut Criterion) {
    let channel = Channel::<u64>::new(1000);
    c.bench_function("channel_send", |b| {
        b.iter(|| channel.send(black_box(42u64)).unwrap());
    });
}

fn bench_channel_recv(c: &mut Criterion) {
    let channel = Channel::<u64>::new(1000);
    channel.send(42u64).unwrap();
    c.bench_function("channel_recv", |b| {
        b.iter(|| black_box(channel.recv()));
    });
}

fn bench_channel_send_recv_batch(c: &mut Criterion) {
    let channel = Channel::<u64>::new(1000);
    c.bench_function("channel_send_recv_batch", |b| {
        b.iter(|| {
            for i in 0..100 {
                let _ = channel.send(i);
            }
            for _ in 0..100 {
                let _ = channel.recv();
            }
        });
    });
}

fn bench_channel_len(c: &mut Criterion) {
    let channel = Channel::<u64>::new(1000);
    for i in 0..100 {
        channel.send(i).unwrap();
    }
    c.bench_function("channel_len", |b| {
        b.iter(|| channel.len());
    });
}

fn bench_channel_is_empty(c: &mut Criterion) {
    let channel = Channel::<u64>::new(1000);
    c.bench_function("channel_is_empty", |b| {
        b.iter(|| channel.is_empty());
    });
}

fn bench_channel_is_full(c: &mut Criterion) {
    let channel = Channel::<u64>::new(10);
    for i in 0..10 {
        channel.send(i).unwrap();
    }
    c.bench_function("channel_is_full", |b| {
        b.iter(|| channel.is_full());
    });
}

fn bench_channel_close(c: &mut Criterion) {
    let channel = Channel::<u64>::new(1000);
    c.bench_function("channel_close", |b| {
        b.iter(|| channel.close());
    });
}

fn bench_ring_buffer_new(c: &mut Criterion) {
    c.bench_function("ring_buffer_new", |b| {
        b.iter(|| RingBuffer::<u64>::new(black_box(1000)));
    });
}

fn bench_ring_buffer_push(c: &mut Criterion) {
    let mut buffer = RingBuffer::<u64>::new(1000);
    c.bench_function("ring_buffer_push", |b| {
        b.iter(|| buffer.push(black_box(42u64)));
    });
}

fn bench_ring_buffer_pop(c: &mut Criterion) {
    let mut buffer = RingBuffer::<u64>::new(1000);
    buffer.push(42u64);
    c.bench_function("ring_buffer_pop", |b| {
        b.iter(|| buffer.pop());
    });
}

fn bench_ring_buffer_push_pop_batch(c: &mut Criterion) {
    let mut buffer = RingBuffer::<u64>::new(1000);
    c.bench_function("ring_buffer_push_pop_batch", |b| {
        b.iter(|| {
            for i in 0..100 {
                buffer.push(i);
            }
            for _ in 0..100 {
                buffer.pop();
            }
        });
    });
}

fn bench_ring_buffer_len(c: &mut Criterion) {
    let mut buffer = RingBuffer::<u64>::new(1000);
    for i in 0..100 {
        buffer.push(i);
    }
    c.bench_function("ring_buffer_len", |b| {
        b.iter(|| buffer.len());
    });
}

fn bench_ring_buffer_is_empty(c: &mut Criterion) {
    let mut buffer = RingBuffer::<u64>::new(1000);
    buffer.push(42u64);
    c.bench_function("ring_buffer_is_empty", |b| {
        b.iter(|| buffer.is_empty());
    });
}

fn bench_work_queue_new(c: &mut Criterion) {
    c.bench_function("work_queue_new", |b| {
        b.iter(|| WorkQueue::<u64>::new());
    });
}

fn bench_work_queue_push(c: &mut Criterion) {
    let queue = WorkQueue::<u64>::new();
    c.bench_function("work_queue_push", |b| {
        b.iter(|| queue.push(black_box(42u64)));
    });
}

fn bench_work_queue_pop(c: &mut Criterion) {
    let queue = WorkQueue::<u64>::new();
    queue.push(42u64);
    c.bench_function("work_queue_pop", |b| {
        b.iter(|| queue.pop());
    });
}

fn bench_work_queue_pop_empty(c: &mut Criterion) {
    let queue = WorkQueue::<u64>::new();
    c.bench_function("work_queue_pop_empty", |b| {
        b.iter(|| queue.pop());
    });
}

fn bench_work_queue_steal(c: &mut Criterion) {
    let queue = WorkQueue::<u64>::new();
    queue.push(42u64);
    c.bench_function("work_queue_steal", |b| {
        b.iter(|| queue.steal());
    });
}

fn bench_work_queue_push_pop_batch(c: &mut Criterion) {
    let queue = WorkQueue::<u64>::new();
    c.bench_function("work_queue_push_pop_batch", |b| {
        b.iter(|| {
            for i in 0..100 {
                queue.push(i);
            }
            for _ in 0..100 {
                queue.pop();
            }
        });
    });
}

criterion_group!(
    benches,
    bench_channel_new,
    bench_channel_send,
    bench_channel_recv,
    bench_channel_send_recv_batch,
    bench_channel_len,
    bench_channel_is_empty,
    bench_channel_is_full,
    bench_channel_close,
    bench_ring_buffer_new,
    bench_ring_buffer_push,
    bench_ring_buffer_pop,
    bench_ring_buffer_push_pop_batch,
    bench_ring_buffer_len,
    bench_ring_buffer_is_empty,
    bench_work_queue_new,
    bench_work_queue_push,
    bench_work_queue_pop,
    bench_work_queue_pop_empty,
    bench_work_queue_steal,
    bench_work_queue_push_pop_batch
);
criterion_main!(benches);
