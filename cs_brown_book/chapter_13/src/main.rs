use std::thread;
fn searchable(list:&Vec<i32> )->bool{
list.contains(&2)
}

fn main() {
    println!("Hello, world!");
    let mut list = vec![1, 2, 3];
    println!("{:?}", list);
    list = thread::spawn(move || {
        println!("{:?}", list);
        list
    })
    .join()
    .unwrap();
    searchable(&list);
    thread::spawn(move || println!("{:?}", list));
    println!();
}
