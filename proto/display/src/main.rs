use rppal::gpio::*;
use std::time::*;
use std::thread::sleep;

// row and column pins in BCM format
const ROW: [u8; 7] = [ 21, 20, 16, 26, 19,13, 6 ];
const COL: [u8; 7] = [ 1, 7, 8, 25, 11, 9, 10 ];
// we use u8 for bit because it is more visual and easy to edit than bool
const PINS_0: [u8; 7] = [ 1, 1, 1, 0, 1, 1, 1 ];
const PINS_1: [u8; 7] = [ 0, 0, 1, 0, 1, 0, 0 ];
const PINS_2: [u8; 7] = [ 1, 0, 1, 1, 0, 1, 1 ];
const PINS_3: [u8; 7] = [ 1, 0, 1, 1, 1, 0, 1 ];
const PINS_4: [u8; 7] = [ 0, 1, 1, 1, 1, 0, 0 ];
const PINS_5: [u8; 7] = [ 1, 1, 0, 1, 1, 0, 1 ];
const PINS_6: [u8; 7] = [ 1, 1, 0, 1, 1, 1, 1 ];
const PINS_7: [u8; 7] = [ 1, 0, 1, 0, 1, 0, 0 ];
const PINS_8: [u8; 7] = [ 1, 1, 1, 1, 1, 1, 1 ];
const PINS_9: [u8; 7] = [ 1, 1, 1, 1, 1, 0, 1 ];
const PINS_X: [u8; 7] = [ 1, 1, 0, 1, 0, 1, 1 ];

struct Display {
    pins_row: [OutputPin; 7],
    pins_col: [OutputPin; 7],
}

impl Display {
    pub fn new(gpio: &Gpio) -> Result<Display> {
        let pinr1 = gpio.get(ROW[0])?.into_output();
        let pinr2 = gpio.get(ROW[1])?.into_output();
        let pinr3 = gpio.get(ROW[2])?.into_output();
        let pinr4 = gpio.get(ROW[3])?.into_output();
        let pinr5 = gpio.get(ROW[4])?.into_output();
        let pinr6 = gpio.get(ROW[5])?.into_output();
        let pinr7 = gpio.get(ROW[6])?.into_output();

        let pinc1 = gpio.get(COL[0])?.into_output();
        let pinc2 = gpio.get(COL[1])?.into_output();
        let pinc3 = gpio.get(COL[2])?.into_output();
        let pinc4 = gpio.get(COL[3])?.into_output();
        let pinc5 = gpio.get(COL[4])?.into_output();
        let pinc6 = gpio.get(COL[5])?.into_output();
        let pinc7 = gpio.get(COL[6])?.into_output();

        let mut pins_row = [pinr1, pinr2, pinr3, pinr4, pinr5, pinr6, pinr7 ];
        let mut pins_col = [pinc1, pinc2, pinc3, pinc4, pinc5, pinc6, pinc7];
        for i in 0..7 {
            pins_col[i].set_high();
            pins_row[i].set_low();
        }

        Ok(Display { pins_row, pins_col })
    }

    fn set_list_high(&mut self, pin_list: Vec<u8>) {
        for i in pin_list {
            self.pins_col[0].set_low();
        }
    }

    fn set_digit(&mut self, value: u8, pos: usize) {
        self.pins_col[pos].set_low();
        let pin_list = match value {
            0 => PINS_0,
            1 => PINS_1,
            2 => PINS_2,
            3 => PINS_3,
            4 => PINS_4,
            5 => PINS_5,
            6 => PINS_6,
            7 => PINS_7,
            8 => PINS_8,
            9 => PINS_9,
            _ => PINS_X,
        };
        for i in 0..7 {
            if pin_list[i] == 1 { 
                self.pins_row[i].set_high();
            }
        }
    }

    fn clear_col(&mut self, pos: usize) {
        self.pins_col[pos].set_high();
        for i in 0..7 {
            self.pins_row[i].set_low();
        }
    }
}

fn main() {
    let gpio = Gpio::new().unwrap();
    let mut display = Display::new(&gpio).unwrap();
    for i in 0..11 {
        display.set_digit(i,0);
        sleep(Duration::from_secs(2));
        display.clear_col(0);
    }
    sleep(Duration::from_secs(2));
}
