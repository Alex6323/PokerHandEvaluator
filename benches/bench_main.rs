use poker_evaluator2;
use poker_evaluator2::models::*;
use poker_evaluator2::eval::*;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

fn temp(c: &mut Criterion) {
    let hand = poker_evaluator2::models::Hand::new("AsAdAhAcKsKcKd");
    //let hand = poker_evaluator2::models::Hand::new("AdAcKhTsTdAh3d");
    c.bench_function("evaluate", move |b| b.iter(|| poker_evaluator2::eval::evaluate(&hand)));
}

criterion_group!(benches, temp);
criterion_main!(benches);