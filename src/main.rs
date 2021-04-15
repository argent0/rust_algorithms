use rust_algorithms::linked_list::List;
use rust_algorithms::linked_list::List::{Nil, Cons};
use rust_algorithms::linked_list;

/// True if the list contains a pair of different elements that add to `total`.
fn has_pair_adding_to(total : i32, xs : &List<i32>) -> bool {
    fn mapper(xs : &List<i32>) -> List<i32> {
        match xs {
            Nil => Nil,
            Cons(x, xss) => linked_list::map(|a| {x + a}, xss)
        }
    }
    let sums : List<i32> = linked_list::concat_map(mapper, &linked_list::tails(xs));
    return linked_list::elem(total, &sums);
}

fn main() {
    let list : List<i32> = linked_list::range(10);
    println!("{:?}", list);
    println!("{}", linked_list::elem(5, &list));
    println!("{}", has_pair_adding_to(17, &list));
    println!("{}", has_pair_adding_to(69, &list));
}
