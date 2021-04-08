use rppal::gpio::*;
use std::time::*;
use std::thread::sleep;
use std::sync::Arc;

/* Keys are all connected to the same pin and just have a different resistance value
 *
 *
 * 
 *           __o__  --------      220nF
 *  V+ ------*   *--|x ohms|---T---||-----\
 *                  --------   |          |
 *                             |          |
 *                          measure      GND   
 *                            pin        
 *
 */

const KEY_PIN: [u8; 2] = [17, 21]; // 0 -> KEY0, 1 -> KEY1
const MAX_PUSH_MS: u64 = 1000;
const MAX_CHARGE_US: u128 = 6000;
const DISCHARGE_MS: u64 = 10;

#[derive(Debug)]
pub enum Button {
    Snooze, B1, B2, Time, SpkrLow, SpkrHigh, Left, Right, OnOff
}

pub struct Keys {
    // pins must change between interrupt, input and output, so we cannot store pins directly
    // so we store a reference to gpio and take pins each time
    gpio: Arc<Gpio>,
    // instead of 2 series of keys
    // pin1: IoPin,
    // pin2: IoPin,
}

impl Keys {
    pub fn new(gpio: Arc<Gpio>) -> Result<Self> {
        let mut keys = Keys { gpio };
        keys.discharge();
        return Ok(keys);
    }

    fn get_input_nopull(&self, keys: usize) -> InputPin {
        let mut pin = self.gpio.get(KEY_PIN[keys]).unwrap().into_input();
        pin.set_reset_on_drop(false);
        return pin;
    }

    fn get_input_pulldown(&self, keys: usize) -> InputPin {
        let mut pin = self.gpio.get(KEY_PIN[keys]).unwrap().into_input_pulldown();
        pin.set_reset_on_drop(false);
        return pin;
    }

    fn get_output(&self, keys: usize) -> OutputPin {
        let mut pin = self.gpio.get(KEY_PIN[keys]).unwrap().into_output();
        pin.set_reset_on_drop(false);
        return pin;
    }

    fn discharge(&mut self) {
        // we must discharge both, otherwise they recharge each other when circuit is open
        self.get_output(0).set_low();
        self.get_output(1).set_low();
        // approx discharge time <1ms (220nF)
        sleep(Duration::from_millis(DISCHARGE_MS));
    }

    fn measure_resistor(&mut self, keys: usize) -> Option<u128> {
        let pin = self.get_input_nopull(keys);
 
        let start = Instant::now();
        // wait for charge
        let mut us = 0;
        while pin.is_low()  {
            sleep(Duration::from_micros(1));
            us += 1;
            if us > MAX_CHARGE_US { break } // about 10ms max charge
        }
        let duration = start.elapsed().as_micros();
        if duration > MAX_CHARGE_US {
            None
        } else {
            Some(duration)
        }
    }

    fn measure_push(&mut self, keys: usize) -> Option<(u128,u128)> {
        // detect fake push and let time for current to establish
        sleep(Duration::from_millis(1));
        if self.get_input_pulldown(keys).is_low() { return None }
    
        let start = Instant::now();
        // which button ?
        self.discharge();
        let resistor1 = self.measure_resistor(keys);
        self.discharge();
        let resistor2 = self.measure_resistor(keys);
        println!("resistor1 {:?}, resistor2 {:?}", resistor1, resistor2);
        let resistor = average(resistor1, resistor2)?;
    
        // wait for release
        {
            let pin = self.get_input_pulldown(keys);
            let mut ms = 0;
            while pin.is_high() {
                sleep(Duration::from_millis(1));
                ms += 1;
                if ms > MAX_PUSH_MS { break } // about 1s timeout
            }
        }
    
        let duration = start.elapsed().as_millis();
        self.discharge();
        
        return Some((resistor, duration));
    }

    #[rustfmt::skip]
    fn detect_button(&mut self, keys: usize) -> Option<Button> {
        let (resistor, time) = self.measure_push(keys)?;
        println!("Pushed {} for {} ms", &resistor, &time);
        if keys == 0 {
                 if in_range(resistor,1700) { Some(Button::Right) } 
            else if in_range(resistor, 850) { Some(Button::Left) }
            else if in_range(resistor, 200) { Some(Button::SpkrHigh) }
            else if in_range(resistor,  15) { Some(Button::SpkrLow) }
            else { println!("resistor {}", resistor); None}
        } else {
                 if in_range(resistor, 4300) { Some(Button::B2) }
            else if in_range(resistor, 1900) { Some(Button::B1) }
            else if in_range(resistor,  950) { Some(Button::Snooze) }
            else if in_range(resistor,  250) { Some(Button::Time) }
            else if in_range(resistor,   15) { Some(Button::OnOff) }
            else { println!("resistor {}", resistor); None }
        }
    }

    pub fn poll_button(&mut self) -> Result<Option<Button>> {
        let keys = {
            let mut pin0 = self.get_input_pulldown(0);    
            let mut pin1 = self.get_input_pulldown(1);    
            pin0.set_interrupt(Trigger::RisingEdge)?;
            pin1.set_interrupt(Trigger::RisingEdge)?;
            let pins = [&pin0, &pin1];
            let res = self.gpio.poll_interrupts(&pins, true, None)?;
            match res {
                None => return Ok(None),
                Some((pin, _)) => if pin == pin0 { 0 } else { 1 }
            }
        };
        println!("interrupted by {}", keys);
        Ok(self.detect_button(keys))
    }
}

// allow variation on resistor measurement
fn in_range(value: u128, expected: u128) -> bool {
    if expected <= 30 { return value < (expected * 2) }
    return value > (expected * 7 / 10 ) && value < (expected * 14 / 10)
}

// average with Option integers
fn average(value1: Option<u128>, value2: Option<u128>) -> Option<u128> {
    match (value1, value2) {
        (None, None) => None,
        (Some(x), None) => Some(x),
        (None, Some(x)) => Some(x),
        (Some(x), Some(y)) => Some((x+y)/2),
    }
}
