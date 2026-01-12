// benches/benchmark.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use trafficforge::plugins::protocols::tcp::{TcpStreamHandler};
use trafficforge::engine::traits::TcpConfig;
use std::time::Duration;

fn bench_tcp_connect(c: &mut Criterion) {
    let config = TcpConfig {
        nodelay: true,
        timeout: Duration::from_secs(30),
        buffer_size: 1024,
    };

    c.bench_function("tcp_handler_creation", |b| {
        b.iter(|| {
            black_box(TcpStreamHandler::new(config.clone()));
        });
    });
}

criterion_group!(benches, bench_tcp_connect);
criterion_main!(benches);