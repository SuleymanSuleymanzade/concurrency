use std::thread;


fn main() {

    println!("FIRST hello 1 from thread");
    println!("FIRST hello 2 from thread");
    thread::spawn( || {
        println!("hello 1 from thread");
        println!("hello 2 from thread");
        println!("hello 3 from thread");
        println!("hello 4 from thread");
        println!("hello 5 from thread");
        println!("hello 6 from thread");
    });
    println!("Outside hello 1 from thread");
    println!("Outside hello 2 from thread");
        
}
