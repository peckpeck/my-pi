use std::thread;
use std::sync::mpsc::channel;
use std::time::*;
use std::thread::sleep;
use chrono::Local;
use std::sync::mpsc;

fn main() {
    // init
    let (tx, rx) = channel();

    // spawn threads
    thread::spawn(move|| { keys_thread(tx) });
    thread::spawn(led_display_thread);
    main_thread(rx);
}

// thread 1 : handle keystrokes
// thread 2 : handle led matrix
// master thread : handle everything else

enum Key { Ok }

fn keys_thread(tx: mpsc::Sender<Key>) {
    loop {
        // wait for key event
        // read key via sleeps
        // send key to master thread via mspc
        sleep(Duration::from_millis(1000));
        tx.send(Key::Ok);
    }
}

fn led_display_thread() {
    loop {
        // read time+options in a rwlock
        // read dimm
        // split by column
        // loop display
        // sleep
        let time = Local::now();
        println!("Time = {}", time.format("%H:%M:%S"));
        sleep(Duration::from_millis(1000));
    }
}

fn main_thread(rx: mpsc::Receiver<Key>) {
    // wait for event : key, timeout
    // keys : snooze 
    // keys : stop / start sound
    // keys : vol +
    // keys : vol -
    // keys : refresh
    // keys : disable alarm
    // keys : enable alarm
    // timeout : update rwlock time
    // timeout : update top clock
    // timeout : run radio / fallback
    // timeout xN : update alarm from calendar
    loop {
        let _ = rx.recv().unwrap();
        println!("key");
    }
}
