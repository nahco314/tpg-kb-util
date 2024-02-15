use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tpg_kb_util::listen_events;

pub fn main() {
    let (tx, rx) = mpsc::channel();
    let t = thread::spawn(move || {
        listen_events(tx);
    });

    for r in rx {
        println!("{:?}", r);
    }
}
