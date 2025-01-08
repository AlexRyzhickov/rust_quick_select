use ordered_float::OrderedFloat;
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::cmp::Ordering;

const LOOPS: u32 = 100;
pub const VEC_SIZE: usize = 1_500_000;
pub const LAST: usize = 50_000;
pub const SEED: u64 = 0;

#[derive(Clone, Copy, Debug)]
pub struct Doc {
    pub id: u32,
    pub score: OrderedFloat<f32>,
}

impl PartialOrd for Doc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.score.cmp(&other.score))
    }
}
impl PartialEq<Self> for Doc {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

fn partition<T: PartialOrd + Copy>(v: &mut [T], mut l: usize, mut r: usize) -> usize {
    let pivot = v[(r + l) / 2];
    while l <= r {
        while v[l] < pivot {
            l += 1;
        }
        while v[r] > pivot {
            r -= 1;
        }
        if l >= r {
            break;
        }
        v.swap(l, r);
        l += 1;
        r -= 1;
    }
    r
}

pub fn quick_select<T: PartialOrd + Copy>(v: &mut [T], l: usize, r: usize, k: usize) {
    if r == l {
        return;
    }
    let mid = partition(v, l, r);
    if k > mid {
        quick_select(v, mid + 1, r, k)
    } else {
        quick_select(v, l, mid, k)
    }
}

fn partition_no_generic(v: &mut [Doc], mut l: usize, mut r: usize) -> usize {
    let pivot = v[(r + l) / 2].score;
    while l <= r {
        while v[l].score < pivot {
            l += 1;
        }
        while v[r].score > pivot {
            r -= 1;
        }
        if l >= r {
            break;
        }
        v.swap(l, r);
        l += 1;
        r -= 1;
    }
    r
}

pub fn quick_select_no_generic(v: &mut [Doc], l: usize, r: usize, k: usize) {
    if r == l {
        return;
    }
    let mid = partition_no_generic(v, l, r);
    if k > mid {
        quick_select_no_generic(v, mid + 1, r, k)
    } else {
        quick_select_no_generic(v, l, mid, k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ordered_float::OrderedFloat;
    use std::collections::HashSet;

    #[test]
    fn test_quick_select_struct() {
        let mut v = vec![
            Doc {
                id: 1,
                score: OrderedFloat(1.0),
            },
            Doc {
                id: 2,
                score: OrderedFloat(5.0),
            },
            Doc {
                id: 2,
                score: OrderedFloat(6.0),
            },
            Doc {
                id: 2,
                score: OrderedFloat(3.0),
            },
            Doc {
                id: 2,
                score: OrderedFloat(2.0),
            },
            Doc {
                id: 2,
                score: OrderedFloat(0.0),
            },
            Doc {
                id: 2,
                score: OrderedFloat(4.0),
            },
            Doc {
                id: 2,
                score: OrderedFloat(9.0),
            },
            Doc {
                id: 2,
                score: OrderedFloat(7.0),
            },
            Doc {
                id: 2,
                score: OrderedFloat(8.0),
            },
        ];
        let last = 3;
        let r = v.len() - 1;
        let k = v.len() - last;
        quick_select(&mut v, 0, r, k);
        let mut set: HashSet<OrderedFloat<f32>> = HashSet::new();
        for &num in v.iter().rev().take(last) {
            set.insert(OrderedFloat::from(num.score));
        }
        v.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        assert_eq!(last, set.len());
        for &num in v.iter().rev().take(last) {
            assert_eq!(set.contains(&OrderedFloat::from(num.score)), true);
        }
    }
    #[test]
    fn test_quick_select_ordered_f32() {
        let mut v = vec![
            OrderedFloat(1.0),
            OrderedFloat(5.0),
            OrderedFloat(6.0),
            OrderedFloat(3.0),
            OrderedFloat(2.0),
            OrderedFloat(0.0),
            OrderedFloat(4.0),
            OrderedFloat(9.0),
            OrderedFloat(7.0),
            OrderedFloat(8.0),
        ];
        let last = 3;
        let r = v.len() - 1;
        let k = v.len() - last;
        quick_select(&mut v, 0, r, k);
        let mut set: HashSet<OrderedFloat<f32>> = HashSet::new();
        for &num in v.iter().rev().take(last) {
            set.insert(OrderedFloat::from(num));
        }
        v.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        assert_eq!(last, set.len());
        for &num in v.iter().rev().take(last) {
            assert_eq!(set.contains(&OrderedFloat::from(num)), true);
        }
    }
    #[test]
    fn test_quick_select_f32() {
        let mut v = vec![1.0, 5.0, 6.0, 3.0, 2.0, 0.0, 4.0, 9.0, 7.0, 8.0];
        let last = 3;
        let r = v.len() - 1;
        let k = v.len() - last;
        quick_select(&mut v, 0, r, k);
        let mut set: HashSet<OrderedFloat<f32>> = HashSet::new();
        for &num in v.iter().rev().take(last) {
            set.insert(OrderedFloat::from(num));
        }
        v.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        assert_eq!(last, set.len());
        for &num in v.iter().rev().take(last) {
            assert_eq!(set.contains(&OrderedFloat::from(num)), true);
        }
    }
    #[test]
    fn test_quick_select_i32() {
        let mut v = vec![1, 5, 6, 3, 2, 0, 4, 9, 7, 8];
        let last = 3;
        let r = v.len() - 1;
        let k = v.len() - last;
        quick_select(&mut v, 0, r, k);
        let mut set: HashSet<i32> = HashSet::new();
        for &num in v.iter().rev().take(last) {
            set.insert(num);
        }
        v.sort_by(|a, b| a.cmp(&b));
        assert_eq!(last, set.len());
        for &num in v.iter().rev().take(last) {
            assert_eq!(set.contains(&num), true);
        }
    }
    #[test]
    fn test_i32_random() {
        for _ in 0..LOOPS {
            let mut v = gen_vec(VEC_SIZE);
            let r = v.len() - 1;
            let k = v.len() - LAST;
            quick_select(&mut v, 0, r, k);
            let mut set: HashSet<i32> = HashSet::new();
            for &num in v.iter().rev().take(LAST) {
                set.insert(num);
            }
            v.sort_by(|a, b| a.cmp(&b));
            assert_eq!(LAST, set.len());
            for &num in v.iter().rev().take(LAST) {
                assert_eq!(set.contains(&num), true);
            }
        }
    }
    #[test]
    fn test_struct_no_generic_random() {
        for _ in 0..LOOPS {
            let mut v = gen_doc_vec(VEC_SIZE);
            let r = v.len() - 1;
            let k = v.len() - LAST;
            quick_select(&mut v, 0, r, k);
            let mut set: HashSet<OrderedFloat<f32>> = HashSet::new();
            for &num in v.iter().rev().take(LAST) {
                set.insert(OrderedFloat::from(num.score));
            }
            v.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
            assert_eq!(LAST, set.len());
            for &num in v.iter().rev().take(LAST) {
                assert_eq!(set.contains(&OrderedFloat::from(num.score)), true);
            }
        }
    }
}

pub fn gen_vec(size: usize) -> Vec<i32> {
    let mut v: Vec<i32> = (0..size).map(|i| 0 + i as i32).collect();
    let mut rng = StdRng::seed_from_u64(SEED);
    v.shuffle(&mut rng);
    v
}

pub fn gen_doc_vec(size: usize) -> Vec<Doc> {
    let mut v: Vec<Doc> = (0..size).map(|i| Doc{ id: 0, score: OrderedFloat::from(i as f32) }).collect();
    let mut rng = StdRng::seed_from_u64(SEED);
    v.shuffle(&mut rng);
    v
}
