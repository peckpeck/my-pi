use rppal::gpio::*;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::*;

use crate::clock_data::*;

/* LED matrix 
 *   - one cell
 *        
 *                V+
 *                |
 *   V- ---------------
 *          \     |
 *           \/\  |
 *           /__\/|
 *              /\|
 *            ||  |
 *            vv  |
 *
 *
 *  7 segments: 
 *
 */

// row and column pins in BCM format
const ROW: [u8; 7] = [16, 13, 12, 6, 5, 7, 8];
const COL: [u8; 7] = [22, 23, 24, 25, 10, 9, 11];

pub struct LedDisplay {
    pins_row: [OutputPin; 7],
    pins_col: [OutputPin; 7],
    display_data: Arc<Mutex<ClockData>>,
}

impl LedDisplay {
    pub fn new(gpio: Arc<Gpio>, display_data: Arc<Mutex<ClockData>>) -> Result<Self> {
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

        let mut pins_row = [pinr1, pinr2, pinr3, pinr4, pinr5, pinr6, pinr7];
        let mut pins_col = [pinc1, pinc2, pinc3, pinc4, pinc5, pinc6, pinc7];
        for i in 0..7 {
            pins_col[i].set_high();
            pins_row[i].set_low();
        }

        Ok(LedDisplay {
            pins_row,
            pins_col,
            display_data,
        })
    }

    fn show_col(&mut self, col: usize) {
        self.pins_col[col].set_low();
        let pins = self.display_data.lock().expect("poisoned mutex 3").get_row_pins_led(col);
        for row in 0..7 {
            if pins[row] == 1 {
                self.pins_row[row].set_high();
            }
        }
    }

    fn clear_col(&mut self, pos: usize) {
        self.pins_col[pos].set_high();
        for i in 0..7 {
            self.pins_row[i].set_low();
        }
    }

    pub fn show(&mut self) {
        let col_wait = Duration::from_micros(self.display_data.lock().expect("poisoned mutex 4").pwm_time(true) / 7);
        for col in 0..7 {
            self.show_col(col);
            sleep(col_wait);
            self.clear_col(col);
        }
        let clear_wait = self.display_data.lock().expect("poisoned mutex 5").pwm_time(false);
        sleep(Duration::from_micros(clear_wait));
    }
}
