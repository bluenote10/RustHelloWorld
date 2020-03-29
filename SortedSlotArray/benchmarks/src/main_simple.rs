extern crate sorted_slot_array;

use rand::Rng;

use slot_array::sorted_array::SortedArray;
use slot_array::array_tree::ArrayTree;
use slot_array::splay::SplaySet;
use slot_array::vec_set::VecSet;

use pretty_assertions::assert_eq;

macro_rules! create_cmp {
    ($func:ident, $get:ident, $count:ident) => {
        static mut $count: u64 = 0;

        fn $func(a: &f64, b: &f64) -> std::cmp::Ordering {
            unsafe {
                $count += 1;
            }
            a.partial_cmp(b).unwrap()
        }

        fn $get() -> u64 {
            unsafe {
                $count
            }
        }
    };
}


create_cmp!(cmp_a, get_num_calls_a, NUM_CALLS_A);
create_cmp!(cmp_b, get_num_calls_b, NUM_CALLS_B);
create_cmp!(cmp_c, get_num_calls_c, NUM_CALLS_C);
create_cmp!(cmp_d, get_num_calls_d, NUM_CALLS_D);


fn main() {

    let mut rng = rand::thread_rng();

    let n = 100;
    let vals: Vec<f64> = (0..n).map(|_| rng.gen()).collect();

    let mut set_a = SplaySet::new(cmp_a);
    let mut set_b = SortedArray::new(cmp_b, 20, 4);
    let mut set_c = VecSet::new(cmp_c, 20);
    let mut set_d = ArrayTree::new(cmp_d, 16);

    for x in &vals {
        set_a.insert(*x);
        set_b.insert(*x);
        set_c.insert(*x);
        set_d.insert(*x);
    }

    set_b.debug();

    let data_a: Vec<_> = set_a.into_iter().collect();
    let data_b = set_b.collect();
    let data_c = set_c.collect();
    let data_d = set_d.collect();

    println!("{:?}", vals);
    assert_eq!(data_a.len(), n);
    assert_eq!(data_b.len(), n);
    assert_eq!(data_c.len(), n);
    assert_eq!(data_d.len(), n);
    assert_eq!(data_a, data_b);
    assert_eq!(data_a, data_c);
    assert_eq!(data_a, data_d);

    println!("Num calls A: {}", get_num_calls_a());
    println!("Num calls B: {}", get_num_calls_b());
    println!("Num calls C: {}", get_num_calls_c());
    println!("Num calls D: {}", get_num_calls_d());

}