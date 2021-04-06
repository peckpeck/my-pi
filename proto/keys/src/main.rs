use rppal::gpio::*;
use std::time::*;
use std::thread::sleep;


fn discharge(gnd: &mut IoPin, pin: &mut IoPin) {
    // pull to ground
    gnd.set_mode(Mode::Output);
    gnd.set_low();
    pin.set_mode(Mode::Output);
    pin.set_low();

    // approx discharge time <1ms (220nF)
    sleep(Duration::from_millis(10));

    // no pull on ground to avoid charging
    gnd.set_mode(Mode::Input);
    gnd.set_pullupdown(PullUpDown::Off);
    // pulldown on pin to detect push
    pin.set_mode(Mode::Input);
    pin.set_pullupdown(PullUpDown::PullDown);

}

fn measure_resistor(gnd: &mut IoPin, pin: &mut IoPin) -> u128 {
    // start charging condensator
    gnd.set_mode(Mode::Output);
    gnd.set_low();
    pin.set_pullupdown(PullUpDown::Off);

    let start = Instant::now();
    // wait for charge
    let mut us = 0;
    while pin.is_low()  {
        sleep(Duration::from_micros(1));
        us += 1;
        if us > 10000 { break }
    }
    return start.elapsed().as_micros();
}

fn measure_push(gnd: &mut IoPin, pin: &mut IoPin) -> Option<(u128,u128)> {
    // detect fake push and let time for current to establish
    sleep(Duration::from_millis(1));
    if pin.is_low() { return None }

    let start = Instant::now();
    // which button ?
    let resistor = measure_resistor(gnd, pin);

    // wait for release
    pin.set_pullupdown(PullUpDown::PullDown);
    let mut ms = 0;
    while pin.is_high() {
        sleep(Duration::from_millis(1));
        ms += 1;
        if ms > 10000 { break }
    }

    let duration = start.elapsed().as_millis();
    discharge(gnd, pin);
    
    return Some((resistor, duration));
}

fn in_range(value: u128, expected: u128) -> bool {
    if expected < 10 { return value < (expected * 2) }
    return value > (expected * 8 / 10 ) && value < (expected * 12 / 10)
}

#[derive(Debug)]
enum Button {
    Snooze, B1, B2, Time, SpkrLow, SpkrHigh, Left, Right, OnOff
}

fn detect_button(gnd: &mut IoPin, pin1: &mut IoPin, pin2: &mut IoPin) -> Option<Button> {
    if pin1.is_high() {
        let (resistor, time) = measure_push(gnd, pin1)?;
        let button = 
                 if in_range(resistor,1700) { Some(Button::Right) }
            else if in_range(resistor, 750) { Some(Button::Left) }
            else if in_range(resistor, 190) { Some(Button::SpkrHigh) }
            else if in_range(resistor,   5) { Some(Button::SpkrLow) }
            else { println!("resistor {}", resistor); None};
        println!("Pushed for {} ms", time);
        return button;
    }
    else if pin2.is_high() {
        let (resistor, time) = measure_push(gnd, pin2)?;
        let button = 
                 if in_range(resistor, 1680) { Some(Button::B1) }
            else if in_range(resistor, 3480) { Some(Button::B2) }
            else if in_range(resistor,  190) { Some(Button::Time) }
            else if in_range(resistor,  750) { Some(Button::Snooze) }
            else { println!("resistor {}", resistor); None };
        println!("Pushed for {} ms", time);
        return button;
    }
    return None;
}

fn main() {
    println!("Hello, world!");

    let gpio = Gpio::new().unwrap();
    let mut pin_g = gpio.get(22).unwrap().into_io(Mode::Input);
    let mut pin_j = gpio.get(23).unwrap().into_io(Mode::Input);
    let mut pin_b = gpio.get(24).unwrap().into_io(Mode::Input);

    discharge(&mut pin_b, &mut pin_j);
    discharge(&mut pin_b, &mut pin_g);
    let mut j_mem = pin_j.is_high();
    let mut g_mem = pin_g.is_high();

    loop {
        let j = pin_j.is_high();
        let g = pin_g.is_high();

        if j != j_mem || g != g_mem {
            let btn = detect_button(&mut pin_b, &mut pin_j, &mut pin_g);
            println!("Pushed ci {:?}", btn);
            j_mem = j;
            g_mem = g;
        }
    }
}
