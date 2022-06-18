use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test(){
    let handle = thread::spawn(||{
       for i in 0..10{
           println!("new thread-1 {}",i);
           thread::sleep(Duration::from_millis(1));
       }
    });

    for i in 0..5 {
        println!("main thread {}",i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

#[test]
fn test1(){
    let v = vec![1,2,3];
    let handle = thread::spawn(move ||{
        println!("hear is vec {:?}",v);
    });

    handle.join().unwrap();
}