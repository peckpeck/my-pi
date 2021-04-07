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
 *                          measure    GND/open   
 *                            pin        pin
 *
 */

const KEY_PIN: [u8; 2] = [20, 21];
const GND_PIN: u8 = 27;
const MAX_PUSH_MS: u64 = 1000;
const MAX_CHARGE_US: u64 = 10000;

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
    gnd: IoPin, // no interrupt needed
}

impl Keys {
    pub fn new(gpio: Arc<Gpio>) -> Result<Self> {
        let gnd = gpio.get(GND_PIN)?.into_io(Mode::Input);
        let mut keys = Keys { gpio, gnd };
        keys.discharge(0);
        keys.discharge(1);
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

    fn open_gnd_pin(&mut self) {
        self.gnd.set_mode(Mode::Input);
        self.gnd.set_pullupdown(PullUpDown::Off);
    }

    fn ground_gnd_pin(&mut self) {
        self.gnd.set_mode(Mode::Output);
        self.gnd.set_low();
    }

    fn discharge(&mut self, keys: usize) {
        self.get_output(keys).set_low();
        self.ground_gnd_pin();
        // approx discharge time <1ms (220nF)
        sleep(Duration::from_millis(10));
    }

    fn measure_resistor(&mut self, keys: usize) -> u128 {
        let pin = self.get_input_nopull(keys);
        self.ground_gnd_pin();
 
        let start = Instant::now();
        // wait for charge
        let mut us = 0;
        while pin.is_low()  {
            sleep(Duration::from_micros(1));
            us += 1;
            if us > MAX_CHARGE_US { break } // about 10ms max charge
        }
        return start.elapsed().as_micros();
    }

    fn measure_push(&mut self, keys: usize) -> Option<(u128,u128)> {
        // detect fake push and let time for current to establish
        sleep(Duration::from_millis(1));
        if self.get_input_pulldown(keys).is_low() { return None }
    
        let start = Instant::now();
        // which button ?
        let resistor = self.measure_resistor(keys);
    
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
        self.discharge(keys);
        
        return Some((resistor, duration));
    }

    #[rustfmt::skip]
    fn detect_button(&mut self, keys: usize) -> Option<Button> {
        let (resistor, time) = self.measure_push(keys)?;
        println!("Pushed {} for {} ms", &resistor, &time);
        if keys == 0 {
                 if in_range(resistor,1700) { Some(Button::Right) }
            else if in_range(resistor, 750) { Some(Button::Left) }
            else if in_range(resistor, 190) { Some(Button::SpkrHigh) }
            else if in_range(resistor,   5) { Some(Button::SpkrLow) }
            else { println!("resistor {}", resistor); None}
        } else {
                 if in_range(resistor, 1680) { Some(Button::B1) }
            else if in_range(resistor, 3480) { Some(Button::B2) }
            else if in_range(resistor,  190) { Some(Button::Time) }
            else if in_range(resistor,  750) { Some(Button::Snooze) }
            else { println!("resistor {}", resistor); None }
        }
    }

    pub fn poll_button(&mut self) -> Result<Option<Button>> {
        let keys = {
            let mut pin0 = self.get_input_pulldown(0);    
            let mut pin1 = self.get_input_pulldown(1);    
            pin0.set_interrupt(Trigger::RisingEdge)?;
            pin1.set_interrupt(Trigger::RisingEdge)?;
            self.open_gnd_pin();
            let pins = [&pin0, &pin1];
            let res = self.gpio.poll_interrupts(&pins, true, None)?;
            match res {
                None => return Ok(None),
                Some((pin, _)) => if pin == pin0 { 0 } else { 1 }
            }
        };
        Ok(self.detect_button(keys))
    }
}

// allow 20% variation on resistor measurement
fn in_range(value: u128, expected: u128) -> bool {
    if expected < 10 { return value < (expected * 2) }
    return value > (expected * 8 / 10 ) && value < (expected * 12 / 10)
}

