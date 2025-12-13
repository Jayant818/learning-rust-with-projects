use std::sync::{Arc, RwLock};

fn main() {
    let websites = vec![
        "https://google.com",
        "http://localhost:3000",
        "https://zomato.com"
    ];

    let success_counter = Arc::new(RwLock::new(0));

    let mut handleVec = Vec::new();

    for sites in websites {
        let counter = success_counter.clone();
        let handle = std::thread::spawn(move ||{
            // checking for wesbite interactiving 
            let mut data = counter.write().unwrap();
            *data +=1;
        });
        handleVec.push(handle);
    }

    for handler in handleVec {
        handler.join().unwrap();
    }

    println!("Total Site that are currently up {}",success_counter.read().unwrap());
}
