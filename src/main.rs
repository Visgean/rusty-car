use linux_embedded_hal::i2cdev::linux::LinuxI2CError;
use crate::motors::MotorControl;
use std::thread::sleep;
use std::time::Duration;

mod motors;

fn main() {
    let mut motor = motors::MotorControl::new().unwrap();

    println!("The car is moving forward");
    motor.move_all_wheels(1000, 1000, 1000, 1000).unwrap();


    println!("The car is moving forward");
    motor.move_all_wheels(1000, 1000, 1000, 1000);
    sleep(Duration::from_secs(1));
    println!("The car is going backwards");
    motor.move_all_wheels(-1000, -1000, -1000, -1000);
    sleep(Duration::from_secs(1));
    println!("The car is turning left");
    motor.move_all_wheels(-1500, -1500, 2000, 2000);
    sleep(Duration::from_secs(1));
    println!("The car is turning right");
    motor.move_all_wheels(2000, 2000, -1500, -1500);
    sleep(Duration::from_secs(1));
    println!("\nEnd of program");
    motor.move_all_wheels(0, 0, 0, 0);

}