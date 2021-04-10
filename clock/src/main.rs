use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::*;
use std::sync::{Arc, Mutex};
use chrono::Local;
use rppal::gpio::Gpio;

mod display;
mod keys;
mod ceiling;
mod clock_data;
mod player;

use display::*;
use keys::*;
use clock_data::*;
use ceiling::*;
use player::*;

fn main() {
    // init
    let (key_tx, main_rx) = channel();
    let display_data = Arc::new(Mutex::new(ClockData::new()));
    let gpio = Arc::new(Gpio::new().unwrap());
    let mut ceiling = Ceiling::new(gpio.clone(), display_data.clone()).unwrap();
    update_time(&display_data);
    display_data.lock().unwrap().ceiling_upwards = false;
    ceiling.set_time();

    // spawn threads
    let ddt = display_data.clone();
    let gpio2 = gpio.clone();
    thread::spawn(move || led_display_thread(gpio2, ddt));
    thread::spawn(move || keys_thread(key_tx, gpio));
    main_thread(main_rx, display_data);
}

// thread 1 : handle keystrokes
// thread 2 : handle led matrix
// master thread : handle everything else

fn keys_thread(tx: mpsc::Sender<Button>, gpio: Arc<Gpio>) {
    println!("Keys");
    let mut keys = Keys::new(gpio).unwrap();
    loop {
        // wait for key event
        // read key via wait on condo charge
        match keys.poll_button() {
            Err(e) => println!("Error {:?}", e),
            Ok(None) => println!("no button"), 
            // send key to master thread via mspc
            Ok(Some(button)) => tx.send(button).unwrap(),
        }
    }
}

fn led_display_thread(gpio: Arc<Gpio>, display_data: Arc<Mutex<ClockData>>) {
    let time = Local::now();
    println!("Time = {}", time.format("%H:%M:%S"));
    update_time(&display_data);
    let mut display = LedDisplay::new(gpio, display_data).unwrap();
    loop {
        display.show();
    }
}

fn update_time(display_data: &Arc<Mutex<ClockData>>) {
    let time = Local::now();
    let mut data = display_data.lock().unwrap();
    // didn't find a better way to get an integer
    data.hours = time.format("%H").to_string().parse::<u8>().unwrap();
    data.minutes = time.format("%M").to_string().parse::<u8>().unwrap();
}

fn main_thread(rx: mpsc::Receiver<Button>, display_data: Arc<Mutex<ClockData>>) {
    // wait for event : key, timeout
    //
    // key snooze : snooze
    // key OnOff: stop / start audio
    // key spkr+/- : vol +/-
    // key time : enable/disable alarm
    // key left/rght: change channel 
    // key B1 : special mode (+led)
    // key B2 : special mode 2 ?
    //
    // special key time : enable/disable ceiling
    // special key left/right : dimm + / -
    // special key : refresh
    // special key B1 : normal mode (or wait 1mn)
    // 
    // timeout : update rwlock time
    // timeout : update top clock
    // timeout : run radio / fallback
    // timeout xN : update alarm from calendar
    let timeout = Duration::from_millis(1000);
    loop {
        match rx.recv_timeout(timeout) {
            Ok(btn) => println!("button {:?}", btn),
            Err(mpsc::RecvTimeoutError::Timeout) => update_time(&display_data),
            Err(_) => println!("Bork"),
        }
    }
}
