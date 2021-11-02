use kd_tree::{left, right};
use rayon::prelude::*;

pub fn sort<
    T: Sized + PartialOrd + PartialEq + core::fmt::Debug + Send,
    V: Sized + Send,
    const DIM: usize,
>(
    mut values: Vec<([T; DIM], V)>,
) -> Vec<([T; DIM], V)> {
    //Add variable for final index
    let mut values: Vec<_> = values.par_drain(..).map(|(a, b)| (a, b, 0usize)).collect();
    //Call Recursive Sort function
    rec_sort(&mut values, 0, 0);

    //Apply final index by sorting by this index
    values.par_sort_by(|(_, _, a), (_, _, b)| {
        // Verify inequality of indices
        assert_ne!(a, b);
        a.cmp(b)
    });

    //Remove index and return
    values.par_drain(..).map(|(a, b, _)| (a, b)).collect()
}

fn rec_sort<T: Sized + PartialOrd + PartialEq + Send, V: Sized + Send, const DIM: usize>(
    values: &mut [([T; DIM], V, usize)],
    dim: usize,
    index: usize,
) {
    //Check dimension
    let dim = dim % DIM;

    // Sort by current dimension
    values.par_sort_by(|(a, _, _), (b, _, _)| a[dim].partial_cmp(&b[dim]).unwrap());

    if values.len() == 1 {
        values[0].2 = index;
        return;
    } else if values.len() == 2 {
        values[1].2 = index;
        rec_sort(&mut values[..1], dim + 1, left(&index));
        return;
    }
    let len = values.len();
    let lv = (len as f64).log2() as usize;
    let last_line_len = len - 2usize.pow(lv as u32) + 1;
    let mid = 2usize.pow(lv as u32) / 2 - 1;
    let mid = if last_line_len < 2usize.pow(lv as u32) / 2 {
        mid + last_line_len
    } else {
        mid + 2usize.pow(lv as u32) / 2
    };

    let (left_slice, rest) = values.split_at_mut(mid);
    let (mid, right_slice) = rest.split_at_mut(1);

    mid.get_mut(0).unwrap().2 = index;
    // 3 times faster for 50000 elements
    [(left_slice, left(&index)), (right_slice, right(&index))]
        .into_par_iter()
        .for_each(|(s, i)| rec_sort(s, dim + 1, i));
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]
    #![allow(deprecated)]
    use crate::sort;
    use kd_tree::{euclid, Node, Tree};
    use rand::Rng;
    use std::ops::AddAssign;
    use std::time::{Duration, Instant};
    type Prng = rand_pcg::Mcg128Xsl64;

    #[test]
    fn fixed_test() {
        let test = sort(vec![
            ([5, 4], 0),
            ([2, 3], 1),
            ([8, 1], 2),
            ([9, 6], 3),
            ([7, 2], 4),
            ([4, 7], 5),
        ]);
        let correct = [
            ([7, 2], 4),
            ([5, 4], 0),
            ([9, 6], 3),
            ([2, 3], 1),
            ([4, 7], 5),
            ([8, 1], 2),
        ];

        assert_eq!(test, correct)
    }

    #[test]
    fn search() {
        let sorted = sort(vec![
            ([5, 4], 0),
            ([2, 3], 1),
            ([8, 1], 2),
            ([9, 6], 3),
            ([7, 2], 4),
            ([4, 7], 5),
        ]);

        let nodes = sorted
            .iter()
            .map(|(p, v)| Node::new(*p, *v))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        //T, V, SIZE, DIM, MAX_LEVEL
        let tree = Tree::<i32, i32, 6, 2, 3>::new(nodes);

        assert_eq!(tree.search(&[5, 3]).val(), &[5, 4]);
        assert_eq!(tree.search(&[9, 7]).val(), &[9, 6]);
        assert_eq!(tree.search(&[3, 9]).val(), &[4, 7]);
        assert_eq!(tree.search(&[3, 0]).val(), &[2, 3]);
    }

    #[ignore]
    #[test]
    fn random_search() {
        let mut rng = Prng::new(0xcafef00dd15ea5e5);
        let mut duration_linear = Duration::from_secs(0);
        let mut duration_tree = Duration::from_secs(0);

        let (iterations, searches) = if cfg!(debug_assertions) {
            (10, 50)
        } else {
            (100, 500)
        };

        #[cfg(debug_assertions)]
        const TREE_SIZE: usize = 5000;
        #[cfg(not(debug_assertions))]
        const TREE_SIZE: usize = 60000;

        #[cfg(debug_assertions)]
        const MAX_LEVEL: usize = 13;
        #[cfg(not(debug_assertions))]
        const MAX_LEVEL: usize = 16;

        for _ in 0..iterations {
            let values: Vec<([f64; 3], i32)> = (0..TREE_SIZE)
                .map(|_| ([rng.gen(), rng.gen(), rng.gen()], 0))
                .collect();
            let sorted = sort(values.clone());
            let nodes: Vec<_> = sorted.iter().map(|(p, v)| Node::new(*p, *v)).collect();
            let tree: Box<Tree<f64, i32, TREE_SIZE, 3, MAX_LEVEL>> = Box::new(Tree {
                nodes: nodes.try_into().unwrap(),
            });
            let search_points: Vec<[f64; 3]> =
                std::iter::repeat_with(|| [rng.gen(), rng.gen(), rng.gen()])
                    .take(searches)
                    .collect();
            for point in search_points {
                let now = Instant::now();
                let closest: f64 = values
                    .iter()
                    .map(|a| euclid(&point, &a.0))
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
                duration_linear.add_assign(now.elapsed());
                let now = Instant::now();
                let closest_node = tree.search(&point);
                duration_tree.add_assign(now.elapsed());
                assert_eq!(closest, euclid(closest_node.val(), &point),);
            }
        }

        println!("Duration Linear Search: {:?}", duration_linear);
        println!("Duration KD Tree Search: {:?}", duration_tree);
    }
}
