use crate::player::Player;

/* On main led 7 segments
 *    0
 *   --
 * 1| 3|2 
 *   --   
 * 5|  |4 
 *   --
 *   6
 */
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

/* On ceiling 7 segments
 *    3
 *   --
 * 2| 6|7.
 *   --  4
 * 1|  |5.
 *   --
 *   0
 */
const CEILING_MANGLE: [usize; 8] = [6, 5, 1, 0, 7, 4, 3, 2];
const CEILING_UPWARDS_MANGLE: [usize; 8] = [0, 2, 4, 6, 7, 1, 3, 5];

pub struct ClockData {
    pub hours: u8,
    pub minutes: u8,
    pub has_alarm: bool,
    pub alarm_enabled: bool,
    pub error: u8,
    pub regular_dim: u8,   // percentage
    pub refresh_rate: u32, // hertz (regular 7 segments and ceiling led)
    pub ceiling_dim: u8,   // percentage
    pub ceiling_upwards: bool,
    pub player: Option<Player>,
}

impl ClockData {
    pub fn new() -> Self {
        ClockData {
            hours: 88,
            minutes: 88,
            has_alarm: false,
            alarm_enabled: true,
            error: 0,
            regular_dim: 50,
            refresh_rate: 100,
            ceiling_dim: 50,
            ceiling_upwards: true,
            player: None,
        }
    }

    fn get_time_pins(&self, pos: usize) -> [u8; 7] {
        let value = match pos {
            0 => (self.hours / 10) as u8,
            1 => (self.hours % 10) as u8,
            2 => (self.minutes / 10) as u8,
            3 => (self.minutes % 10) as u8,
            _ => 10,
        };
        match value {
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
        }
    }

    fn mangle_pins(&self, pos: usize, mangle: &[usize; 8]) -> [u8; 8] {
        let mut pins = [0; 8];
        let pin_list = self.get_time_pins(pos);
        for i in mangle {
            if mangle[*i] == 7 {
                pins[*i] = 0;
            } else {
                pins[*i] = pin_list[mangle[*i]];
            }
        }
        return pins;
    }

    pub fn get_row_pins_led(&self, col: usize) -> [u8; 7] {
        match col {
            0 => self.get_time_pins(0),
            1 => self.left_opts(),
            2 => self.get_time_pins(1),
            3 => [0, 1, 0, 0, 0, 1, 0],
            4 => self.get_time_pins(2),
            5 => self.right_opts(),
            6 => self.get_time_pins(3),
            _ => [0; 7],
        }
    }

    pub fn get_row_pins_ceiling(&self, col: usize) -> [u8; 8] {
        if self.ceiling_upwards {
            self.mangle_pins(col, &CEILING_MANGLE)
        } else {
            self.mangle_pins(col, &CEILING_UPWARDS_MANGLE)
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

    pub fn pwm_time(&self, up: bool) -> u64 {
        let pct = if up {
            self.regular_dim as u64
        } else {
            100 - self.regular_dim as u64
        };
        return (1_000_000 / self.refresh_rate as u64) * pct / 100;
    }
}

