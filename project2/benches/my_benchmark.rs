use std::iter;

#[path = "../src/avl.rs"]
mod avl;
use crate::avl::AVLTree;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn bench_avl(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_avl");

    for tree_size in [10000, 40000, 70000, 100000, 130000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(tree_size),
            tree_size,
            |b, &tree_size| b.iter(|| insert_contains_avl_tree(tree_size)),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_avl);
criterion_main!(benches);

fn insert_contains_avl_tree(n: i32) {
    let mut avl_tree = AVLTree::new();
    for i in 0..n {
        avl_tree.insert(i);
    }

    // Check for random values in the AVL tree
    for i in 0..(n/10) {
        avl_tree.contains(i);
    }
}