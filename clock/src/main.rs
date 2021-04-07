use chrono::Local;
use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::*;
use std::sync::{Arc, Mutex};

use rppal::gpio::Gpio;
mod display;
use display::*;

fn main() {
    // init
    let (key_tx, main_rx) = channel();
    let gpio = Gpio::new().unwrap();
    let display_data = Arc::new(Mutex::new(DisplayData::new()));

    // spawn threads
    thread::spawn(move || keys_thread(key_tx));
    thread::spawn(move || led_display_thread(&gpio, display_data.clone()));
    main_thread(main_rx);
    //main2();
}

// thread 1 : handle keystrokes
// thread 2 : handle led matrix
// master thread : handle everything else

enum Key {
    Ok,
}

fn keys_thread(tx: mpsc::Sender<Key>) {
    loop {
        // wait for key event
        // read key via sleeps
        // send key to master thread via mspc
        sleep(Duration::from_millis(1000));
        tx.send(Key::Ok);
    }
}

fn led_display_thread(gpio: &Gpio, display_data: Arc<Mutex<DisplayData>>) {
    { 
        let time = Local::now();
        println!("Time = {}", time.format("%H:%M:%S"));
        let mut data = display_data.lock().unwrap();
        // didn't find a better way to get an integer
        data.hours = time.format("%H").to_string().parse::<u8>().unwrap();
        data.minutes = time.format("%M").to_string().parse::<u8>().unwrap();
    }
    let mut display = LedDisplay::new(gpio, display_data).unwrap();
    loop {
        display.show();
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
