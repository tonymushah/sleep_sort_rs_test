use std::{thread, time::Duration};

fn sleepsort<I: Iterator<Item = u32>>(nums: I) {
    let threads: Vec<_> = nums
        .map(|n| {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(n.into()));
                println!("{}", n);
            })
        })
        .collect();
    for t in threads {
        let _ = t.join();
    }
}

fn main() {
    sleepsort(std::env::args().skip(1).map(|s| s.parse().unwrap()));
}
