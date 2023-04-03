use core::fmt::Debug;
use std::iter;

#[path = "../src/avl.rs"]
mod avl;
#[path = "../src/rb.rs"]
mod rb;
use crate::avl::AVLTree;
use crate::rb::RedBlackTree;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
fn bench_rb(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_rb");

    for tree_size in [10000, 40000, 70000, 100000, 130000].iter() {
        let mut rb_tree: RedBlackTree = RedBlackTree::new();

        // These will pass the AVL tree to do benchmarking by first benching the inserts, then benching the searches
        group.bench_with_input(
            BenchmarkId::new("insert", tree_size),
            tree_size,
            |b, &tree_size| b.iter(|| insert_rb_tree(&mut rb_tree, tree_size)),
        );
        group.bench_with_input(
            BenchmarkId::new("search", tree_size),
            tree_size,
            |b, &tree_size| b.iter(|| contains_rb_tree(&mut rb_tree, tree_size)),
        );
    }
    group.finish();
}

fn contains_rb_tree(rb_tree: &mut RedBlackTree, n: u32) {
    for i in 0..(n / 10) {
        rb_tree.contains(i.into());
    }
}

fn insert_rb_tree(rb_tree: &mut RedBlackTree, n: u32) {
    for i in 0..n {
        rb_tree.tree_insert(i.into());
    }
}
fn bench_avl<
    T: std::default::Default
        + std::clone::Clone
        + std::fmt::Debug
        + std::cmp::Ord
        + std::convert::From<i32>,
>(
    c: &mut Criterion,
) {
    let mut group = c.benchmark_group("bench_avl");

    for tree_size in [10000, 40000, 70000, 100000, 130000].iter() {
        let mut avl_tree: AVLTree<T> = AVLTree::new();

        // These will pass the AVL tree to do benchmarking by first benching the inserts, then benching the searches
        group.bench_with_input(
            BenchmarkId::new("insert", tree_size),
            tree_size,
            |b, &tree_size| b.iter(|| insert_avl_tree(&mut avl_tree, tree_size)),
        );
        group.bench_with_input(
            BenchmarkId::new("search", tree_size),
            tree_size,
            |b, &tree_size| b.iter(|| contains_avl_tree(&mut avl_tree, tree_size)),
        );
    }
    group.finish();
}

fn contains_avl_tree<T: std::cmp::Ord + std::convert::From<i32>>(avl_tree: &mut AVLTree<T>, n: i32)
where
    T: Clone,
    T: Debug,
    T: Default,
{
    for i in 0..(n / 10) {
        avl_tree.contains(i.into());
    }
}

fn insert_avl_tree<T: std::cmp::Ord + std::convert::From<i32>>(avl_tree: &mut AVLTree<T>, n: i32)
where
    T: Clone,
    T: Debug,
    T: Default,
{
    for i in 0..n {
        avl_tree.insert(i.into());
    }
}

criterion_group!(benches, bench_avl::<i32>, bench_rb);
criterion_main!(benches);
