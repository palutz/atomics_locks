use std::thread;

fn main() {
    println!("hello from thread {:?}", thread::current().id());
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    t1.join().unwrap();
    t2.join().unwrap();
}

fn f() {
    let id = thread::current().id();
    println!("hello from thread {:?}", id);
}
