use rppal::gpio::*;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::*;

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
 */

// row and column pins in BCM format
const ROW: [u8; 7] = [16, 13, 12, 6, 5, 7, 8];
const COL: [u8; 7] = [22, 23, 24, 25, 10, 9, 11];
// we use u8 for bits because it is more visual and easy to edit than bool
const PINS_0: [u8; 7] = [1, 1, 1, 0, 1, 1, 1];
const PINS_1: [u8; 7] = [0, 0, 1, 0, 1, 0, 0];
const PINS_2: [u8; 7] = [1, 0, 1, 1, 0, 1, 1];
const PINS_3: [u8; 7] = [1, 0, 1, 1, 1, 0, 1];
const PINS_4: [u8; 7] = [0, 1, 1, 1, 1, 0, 0];
const PINS_5: [u8; 7] = [1, 1, 0, 1, 1, 0, 1];
const PINS_6: [u8; 7] = [1, 1, 0, 1, 1, 1, 1];
const PINS_7: [u8; 7] = [1, 0, 1, 0, 1, 0, 0];
const PINS_8: [u8; 7] = [1, 1, 1, 1, 1, 1, 1];
const PINS_9: [u8; 7] = [1, 1, 1, 1, 1, 0, 1];
const PINS_X: [u8; 7] = [1, 1, 0, 1, 0, 1, 1];

pub enum TimeOption {
    HasAlarm,
    HasNoAlarm,
    EnableAlarm,
    DisableAlarm,
    Error(u8),
}

pub struct DisplayData {
    pub hours: u8,
    pub minutes: u8,
    pub has_alarm: bool,
    pub alarm_enabled: bool,
    pub error: u8,
    pub dim: u8,           // percentage
    pub refresh_rate: u32, // hertz
}

impl DisplayData {
    pub fn new() -> DisplayData {
        DisplayData {
            hours: 88,
            minutes: 88,
            has_alarm: false,
            alarm_enabled: true,
            error: 0,
            dim: 50,
            refresh_rate: 100,
        }
    }

    pub fn get_time_pins(&self, pos: usize, upwards: bool) -> [u8; 7] {
        let value = match pos {
            0 => (self.hours / 10) as u8,
            1 => (self.hours % 10) as u8,
            2 => (self.minutes / 10) as u8,
            3 => (self.minutes % 10) as u8,
            _ => 10,
        };
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
        if upwards {
            return pin_list;
        } else {
            return [
                        pin_list[6],
                        pin_list[5],
                        pin_list[4],
                        pin_list[3],
                        pin_list[2],
                        pin_list[1],
                        pin_list[0],
                    ];
        }
    }

    pub fn get_row_pins(&self, col: usize, upwards: bool) -> [u8; 7] {
        match col {
            0 => self.get_time_pins(0, upwards),
            1 => self.left_opts(),
            2 => self.get_time_pins(1, upwards),
            3 => [0, 1, 0, 0, 0, 1, 0],
            4 => self.get_time_pins(2, upwards),
            5 => self.right_opts(),
            6 => self.get_time_pins(3, upwards),
            _ => [0; 7],
        }
    }

    fn left_opts(&self) -> [u8; 7] {
        let mut result = [0; 7];
        // left opts are alarm related
        if self.has_alarm {
            result[5] = 1;
        }
        if !self.alarm_enabled {
            result[1] = 1;
        }
        return result;
    }

    #[rustfmt::skip]
    fn right_opts(&self) -> [u8; 7] {
        let mut result = [0; 7];
        // right opts are error related
        let err = if self.error < 16 { self.error } else { 15 };
        if  err      % 2 == 1 { result[5] = 1; }
        if (err / 2) % 2 == 1 { result[4] = 1; }
        if (err / 4) % 2 == 1 { result[2] = 1; }
        if (err / 8) % 2 == 1 { result[1] = 1; }
        return result;
    }

    fn pwm_time(&self, up: bool) -> u64 {
        let pct = if up {
            self.dim as u64
        } else {
            100 - self.dim as u64
        };
        return (1_000_000 / self.refresh_rate as u64) * pct / 100;
    }
}

pub struct LedDisplay {
    pins_row: [OutputPin; 7],
    pins_col: [OutputPin; 7],
    display_data: Arc<Mutex<DisplayData>>,
}

impl LedDisplay {
    pub fn new(gpio: Arc<Gpio>, display_data: Arc<Mutex<DisplayData>>) -> Result<Self> {
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
        let pins = self.display_data.lock().unwrap().get_row_pins(col, true);
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
        let col_wait = Duration::from_micros(self.display_data.lock().unwrap().pwm_time(true) / 7);
        for col in 0..7 {
            self.show_col(col);
            sleep(col_wait);
            self.clear_col(col);
        }
        let clear_wait = self.display_data.lock().unwrap().pwm_time(false);
        sleep(Duration::from_micros(clear_wait));
    }
}
