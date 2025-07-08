use std::thread;
use std::sync::{Arc,Mutex};

fn main(){

    let counter = Arc::new(Mutex::new(1));

    let mut handles = vec![];

    for _ in 1..10{
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        } );
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
    println!("sonuc {}",*counter.lock().unwrap());

}

