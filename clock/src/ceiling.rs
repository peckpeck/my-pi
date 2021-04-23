use rppal::gpio::*;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::*;

use crate::clock_data::*;


/*
 *  Connector -> real function
 *   _
 *  |.| LED --- Red wire
 *  |.| 3V3
 *   .| GND
 *   .| DATA2 -> LINE
 *   .| WR    -> CLOCK
 *  |.| CLK   -> DATA
 *  |.| GND
 *   -
 *
 */ 

const LED_PIN: u8 = 15;  // LED
const LINE_PIN: u8 = 14; // DATA2
const CLOCK_PIN: u8 = 3; // WR
const DATA_PIN: u8 = 2;  // CLK   

static UP_DURATION: Duration = Duration::from_micros(10);
static DOWN_DURATION: Duration = Duration::from_micros(5);
static BREAK_DURATION: Duration = Duration::from_micros(70);

const BYTE1: [u8; 12] = [1,0,0,0,0,1,0,1,0,0,1,0];
const BYTE2: [u8; 12] = [1,0,0,0,0,0,0,0,0,0,1,0];
const BYTE3: [u8; 12] = [1,0,0,0,0,0,0,0,0,1,1,0];
const BYTE4: [u8; 9] = [1,0,1,0,1,1,0,0,0];

pub struct Ceiling {
    led: OutputPin, 
    data: OutputPin,
    clock: OutputPin,
    line: OutputPin,
    display_data: Arc<Mutex<ClockData>>,
}

impl Ceiling {
    pub fn new(gpio: Arc<Gpio>, display_data: Arc<Mutex<ClockData>>) -> Result<Self> {
        let led = gpio.get(LED_PIN)?.into_output();
        let data = gpio.get(DATA_PIN)?.into_output();
        let clock = gpio.get(CLOCK_PIN)?.into_output();
        let line = gpio.get(LINE_PIN)?.into_output();
        Ok(Ceiling { led, data, clock, line, display_data })
    }

    pub fn set_time(&mut self) {
        let mut data = [0; 41];
        {
            let ddt = self.display_data.lock().expect("poisoned mutex 1");
            let order = if ddt.ceiling_upwards {
                [ 0, 1, 2, 3 ]
            } else {
                [ 3, 2, 1, 0]
            };
            data[0..9].copy_from_slice(&BYTE4);
            data[9..17].copy_from_slice(&ddt.get_row_pins_ceiling(order[0]));
            data[17..25].copy_from_slice(&ddt.get_row_pins_ceiling(order[1]));
            data[21] = 1; // dots
            data[25..33].copy_from_slice(&ddt.get_row_pins_ceiling(order[2]));
            data[33..41].copy_from_slice(&ddt.get_row_pins_ceiling(order[3]));
        }
        self.write_sequence(&BYTE1);
        self.write_sequence(&BYTE2);
        self.write_sequence(&BYTE3);
        self.write_sequence(&data);
    }

    pub fn set_light(&mut self) {
        println!("remove");
        self.led.set_high();
        let ddt = self.display_data.lock().expect("poisoned mutex 2");
        let level = ddt.ceiling_dim as f64;
        let frequency = ddt.refresh_rate as f64;
        self.led.set_pwm_frequency(frequency, level/100.);
    }

    fn write_sequence(&mut self, bits: &[u8]) {
        self.line.set_low();
        sleep(UP_DURATION);
        for bit in bits {
            self.write_bit(*bit);
        }
        self.line.set_high();
        sleep(BREAK_DURATION);
    }

    fn write_bit(&mut self, bit: u8) {
        if bit == 0 { 
            self.data.set_low();
        } else {
            self.data.set_high();
        }
        self.clock.set_low();
        sleep(DOWN_DURATION);
        self.clock.set_high();
        sleep(UP_DURATION);

    }
}
