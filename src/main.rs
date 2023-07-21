#![allow(dead_code, unused)]
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    reverse_collection(&mut numbers);
    println!("{:?}", numbers);
}
fn reverse_collection<T: Clone>(collection: &mut [T]) {
    // Way 1:
    // let mut reverse: Vec<T> = Vec::new();
    // for document in collection.iter().rev() {
    //     reverse.push(document.clone());
    // }
    // collection.clone_from_slice(&reverse);

    // Way 2:
    // let len = collection.len();
    // for i in 0..len / 2 {
    //     collection.swap(i, len - i - 1);
    // }

    collection.reverse();
}
