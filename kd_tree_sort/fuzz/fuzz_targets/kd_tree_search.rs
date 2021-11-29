#![no_main]
use kd_tree::{euclid, Node, Tree};
use kd_tree_sort::sort;
use libfuzzer_sys::fuzz_target;
use rayon::prelude::*;

#[derive(Clone, Debug, arbitrary::Arbitrary)]
struct ArbitraryPoint {
    pos: [f64; 3],
    p: i32,
}

unsafe impl Sync for ArbitraryPoint{}
unsafe impl Send for ArbitraryPoint{}

fn linear_search(nodes: &[Node<f64, i32, 3>], point: &[f64; 3]) -> f64 {
    nodes
        .iter()
        .map(|a| euclid(point, a.val()))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

const MAX_LEVEL: usize = 16;

fuzz_target!(|data: Vec<ArbitraryPoint>| {
    let mut data = data;
    let mut data: Vec<_> = data
        .drain(..)
        .filter(|p| p.pos[0].is_finite() && p.pos[1].is_finite() && p.pos[2].is_finite())
        .take((2usize.pow(MAX_LEVEL as u32) - 1) * 2)
        .par_bridge()
        .map(|p| (p.pos, p.p))
        .collect();

    if data.len() < 2 {
        // Dont fuzzy test less than two elements
        return;
    }

    let search_data: Vec<_> = data.split_off(data.len() / 2);

    let sorted = sort(data);
    let sorted: Vec<_> = sorted.iter().map(|(p, v)| Node::new(*p, *v)).collect();
    let tree = Tree::<f64, i32, 3, MAX_LEVEL> {
        nodes: sorted.as_slice(),
    };

    search_data.par_iter().for_each(|(p, _)| {
        assert_eq!(
            euclid(tree.search(&p).val(), &p),
            linear_search(&sorted, &p)
        )
    });
});
