use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use kd_tree::{euclid, Node, Tree};
use kd_tree_sort::sort;
use rand::Rng;
use std::time::Duration;

type Prng = rand_pcg::Mcg128Xsl64;

fn linear_search(nodes: &[Node<f64, i32, 3>], point: &[f64; 3]) -> f64 {
    nodes
        .iter()
        .map(|a| euclid(point, a.val()))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

fn generate_values(len: usize, rng: &mut Prng) -> Vec<([f64; 3], i32)> {
    (0..len).map(|_| (generate_point(rng), 0)).collect()
}

fn generate_sorted(len: usize, rng: &mut Prng) -> Vec<Node<f64, i32, 3>> {
    let sorted = sort(generate_values(len, rng));
    sorted.iter().map(|(p, v)| Node::new(*p, *v)).collect()
}

fn generate_point(rng: &mut Prng) -> [f64; 3] {
    [rng.gen(), rng.gen(), rng.gen()]
}

pub fn search(c: &mut Criterion) {
    let mut rng = Prng::new(0xcafef00dd15ea5e5);

    const START_LEN: usize = 1000;
    const END_LEN: usize = 64001;

    const MAX_LEVEL: usize = 16;

    let mut group = c.benchmark_group("Search");
    group.warm_up_time(Duration::from_millis(50));
    group.measurement_time(Duration::from_secs(1));

    for len in (START_LEN..END_LEN).step_by(1000) {
        let nodes = generate_sorted(len, &mut rng);
        let tree = Tree::<f64, i32, 3, MAX_LEVEL> {
            nodes: nodes.as_slice(),
        };
        group.bench_with_input(BenchmarkId::new("KD Tree", len), &tree, |b, t| {
            b.iter_batched(
                || generate_point(&mut rng),
                |p| t.search(&p),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("Linear", len), &nodes, |b, n| {
            b.iter_batched(
                || generate_point(&mut rng),
                |p| linear_search(n, &p),
                BatchSize::SmallInput,
            )
        });
    }
}

criterion_group!(benches, search);
criterion_main!(benches);
