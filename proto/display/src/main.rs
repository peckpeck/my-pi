use rppal::gpio::*;
use std::time::*;
use std::thread::sleep;

// row and column pins in BCM format
const ROW: [u8; 7] = [ 21, 20, 16, 26, 19,13, 6 ];
const COL: [u8; 7] = [ 10, 9, 11, 25, 8, 7, 1 ];
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

enum TimeOption {
    HasAlarm,
    HasNoAlarm,
    EnableAlarm,
    DisableAlarm,
    Error(u8),
}

struct Display {
    pins_row: [OutputPin; 7],
    pins_col: [OutputPin; 7],
    digits: [u8; 4],
    has_alarm: bool,
    alarm_enabled: bool,
    error: u8,
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

        Ok(Display { pins_row, pins_col, digits: [0,0,0,0], has_alarm: false, alarm_enabled: true, error: 0 })
    }

    fn show_digit(&mut self, value: u8, pos: usize) {
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

    fn show_colon(&mut self) {
        self.pins_col[3].set_low();
        self.pins_row[1].set_high();
        self.pins_row[5].set_high();
    }

    fn show_left_opts(&mut self) {
        // left opts are alarm related
        self.pins_col[1].set_low();
        if self.has_alarm { self.pins_row[5].set_high(); }
        if !self.alarm_enabled { self.pins_row[1].set_high(); }
    }

    fn show_right_opts(&mut self) {
        // right opts are error related
        self.pins_col[5].set_low();
        let err = if self.error < 16 { self.error } else { 15 };
        if err % 2 == 1 { self.pins_row[5].set_high(); }
        if (err/2) % 2 == 1 { self.pins_row[4].set_high(); }
        if (err/4) % 2 == 1 { self.pins_row[2].set_high(); }
        if (err/8) % 2 == 1 { self.pins_row[1].set_high(); }
    }

    fn clear_col(&mut self, pos: usize) {
        self.pins_col[pos].set_high();
        for i in 0..7 {
            self.pins_row[i].set_low();
        }
    }
    
    pub fn set_time(&mut self, hours: i32, minutes: i32) {
        self.digits[0] = (hours / 10) as u8;
        self.digits[1] = (hours % 10) as u8;
        self.digits[2] = (minutes / 10) as u8;
        self.digits[3] = (minutes % 10) as u8;
    }

    pub fn set_opts(&mut self, option: TimeOption) {
        match option {
            TimeOption::HasAlarm => self.has_alarm = true,
            TimeOption::HasNoAlarm => self.has_alarm = false,
            TimeOption::EnableAlarm => self.alarm_enabled = true,
            TimeOption::DisableAlarm => self.alarm_enabled = false,
            TimeOption::Error(e) => self.error = e,
        }
    }

    pub fn show(&mut self, us_per_col: u64) {
        for i in 0..4 {
            let pos = 2*i;
            self.show_digit(self.digits[i], pos);
            sleep(Duration::from_micros(us_per_col));
            self.clear_col(pos);
        }
        self.show_colon();
        sleep(Duration::from_micros(us_per_col));
        self.clear_col(3);
        self.show_left_opts();
        sleep(Duration::from_micros(us_per_col));
        self.clear_col(1);
        self.show_right_opts();
        sleep(Duration::from_micros(us_per_col));
        self.clear_col(5);
    }
}

fn main() {
    let gpio = Gpio::new().unwrap();
    let mut display = Display::new(&gpio).unwrap();
    display.set_time(12,34);
    display.set_opts(TimeOption::HasAlarm);
    display.set_opts(TimeOption::Error(3));
    loop {
        display.show(1000);
    }
}
