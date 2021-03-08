use linux_embedded_hal::I2cdev;
use linux_embedded_hal::i2cdev::core::I2CDevice;
use linux_embedded_hal::i2cdev::linux::LinuxI2CError;
use std::cmp::{min, max};
use std::thread::sleep;
use std::time::Duration;



#[derive(Debug)]
pub enum Wheel {
    LeftUpper,
    LeftLower,
    RightUpper,
    RightLower,
}

pub const MAX_WHEEL_SPEED: i32 = 4095;
pub const MIN_WHEEL_SPEED: i32 = -4095;

// there is no point in running the motor at these speeds to it is better to simply turn it off
pub const MIN_SPEED_THRESHOLD: i32 = 30;

const MODE1: u8 = 0x00;
const PRESCALE: u8 = 0xFE;
const LED0_ON_L: u8 = 0x06;
const LED0_ON_H: u8 = 0x07;
const LED0_OFF_L: u8 = 0x08;
const LED0_OFF_H: u8 = 0x09;


// See left_Upper_Wheel - it is obvious that the wheel is controlled by two registers
// that corresponds to forward and backwards motion

struct WheelControl {
    forward: u8,
    backward: u8,
}

impl WheelControl {
    pub fn get_control(wheel: Wheel) -> Self {
        match wheel {
            Wheel::LeftUpper => { WheelControl { forward: 1, backward: 0 } }
            Wheel::LeftLower => { WheelControl { forward: 2, backward: 3 } }
            Wheel::RightUpper => { WheelControl { forward: 7, backward: 6 } }
            Wheel::RightLower => { WheelControl { forward: 5, backward: 4 } }
        }
    }
}

pub struct MotorControl {
    dev: I2cdev,
}

impl MotorControl {
    pub fn new() -> Result<Self, LinuxI2CError> {
        let mut dev = I2cdev::new("/dev/i2c-1").unwrap();
        dev.set_slave_address(0b100_0000).unwrap();
        let mut this = Self { dev };

        this.set_frequency();

        Ok(this)
    }

    fn set_frequency(&mut self) -> Result<(), LinuxI2CError> {
        // see setPWMFreq(freq=50) function in the vendor code
        println!("Setting motor frequency 50hz");

        // let prescale: u8 = 121;
        // let oldmode = self.dev.smbus_read_byte_data(MODE1).unwrap();
        // let newmode = (oldmode & 0x7F) | 0x10;
        //
        // self.write(0, 0)?;
        // self.write(MODE1, newmode)?;
        // self.write(PRESCALE, prescale)?;
        // self.write(MODE1, oldmode)?;
        // sleep(Duration::from_millis(5));
        // self.write(MODE1, oldmode | 0x80)?;

        self.write(0, 16)?;
        self.write(254, 121)?;
        self.write(0, 0)?;
        self.write(0, 128)?;


        Ok(())
    }


    fn limit_duty(&mut self, duty: i32) -> i32 {
        // set boundaries at which wheelies can operate
        max(min(duty, MAX_WHEEL_SPEED), MIN_WHEEL_SPEED)
    }

    fn write(&mut self, channel: u8, val: u8) -> Result<(), LinuxI2CError> {
        // write to a single resister
        println!("reg: {} = {}", channel, val);

        self.dev.smbus_write_byte_data(channel, val)
    }

    fn set_motor_power(&mut self, channel: u8, duty: i32) -> Result<(), LinuxI2CError> {
        // fixme: maybe duty should u32?

        println!("Setting motor power: {} = {}", channel, duty);
        self.write(LED0_ON_L + 4 * channel, 0)?;
        self.write(LED0_ON_H + 4 * channel, 0)?;
        self.write(LED0_OFF_L + 4 * channel, duty as u8)?;
        self.write(LED0_OFF_H + 4 * channel, (duty >> 8) as u8)?;

        Ok(())
    }


    pub fn set_wheel_speed(&mut self, wheel: Wheel, mut duty: i32) -> Result<(), LinuxI2CError> {
        println!("Setting wheel {:#?}, speed to {} ", wheel, duty);

        if duty != 0 && duty < MIN_SPEED_THRESHOLD && duty > -MIN_SPEED_THRESHOLD {
            println!("Duty bellow activation threshold, setting to 0");
            duty = 0;
        }

        let control = WheelControl::get_control(wheel);
        duty = self.limit_duty(duty);

        // FIXME: why is this being set to max instead of 0?
        if duty == 0 {
            self.set_motor_power(control.forward, MAX_WHEEL_SPEED)?;
            self.set_motor_power(control.backward, MAX_WHEEL_SPEED)?;
        } else if duty > 0 {
            self.set_motor_power(control.backward, 0)?;
            self.set_motor_power(control.forward, duty)?;
        } else if duty < 0 {
            self.set_motor_power(control.forward, 0)?;
            self.set_motor_power(control.backward, duty.abs())?;
        }

        Ok(())
    }

    pub fn move_all_wheels(&mut self, left_upper: i32, left_lower: i32, right_upper: i32, right_lower: i32) -> Result<(), LinuxI2CError> {
        self.set_wheel_speed(Wheel::LeftUpper, left_upper)?;
        self.set_wheel_speed(Wheel::LeftLower, left_lower)?;
        self.set_wheel_speed(Wheel::RightUpper, right_upper)?;
        self.set_wheel_speed(Wheel::RightLower, right_lower)?;

        Ok(())
    }
}

