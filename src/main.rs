use quick_select::quick_select;
use quick_select::Doc as Doc;
use ordered_float::OrderedFloat;

fn main() {
    let mut v = vec![
        Doc { id: 1, score: OrderedFloat(1.0) },
        Doc { id: 2, score: OrderedFloat(5.0) },
        Doc { id: 2, score: OrderedFloat(6.0) },
        Doc { id: 2, score: OrderedFloat(3.0) },
        Doc { id: 2, score: OrderedFloat(2.0) },
        Doc { id: 2, score: OrderedFloat(0.0) },
        Doc { id: 2, score: OrderedFloat(4.0) },
        Doc { id: 2, score: OrderedFloat(9.0) },
        Doc { id: 2, score: OrderedFloat(7.0) },
        Doc { id: 2, score: OrderedFloat(8.0) },
    ];
    let last = 3;
    let r = v.len() - 1;
    let k = v.len() - last;
    quick_select(&mut v, 0, r, k);
    println!("{:?}", v);
}
