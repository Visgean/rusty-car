use linux_embedded_hal::i2cdev::linux::LinuxI2CError;
use crate::motors::MotorControl;
use std::thread::sleep;
use std::time::Duration;
use gilrs::{Gilrs, Button, Event, Axis};
use gilrs::ev::state::GamepadState;

mod motors;


fn controller_loop(mut motor: MotorControl) {
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut active_gamepad = None;

    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            active_gamepad = Some(id);

            // You can also use cached gamepad state
            if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
                let max_speed = motors::MAX_WHEEL_SPEED as f32;

                let right_side = (gamepad.value(Axis::RightStickY) * max_speed) as i32;
                let left_side = (gamepad.value(Axis::LeftStickY) * max_speed) as i32;


                motor.move_all_wheels(-left_side, -left_side, -right_side, -right_side).unwrap();
            }
        }
    }
}


fn main() {
    let mut motor = motors::MotorControl::new().unwrap();
    controller_loop(motor)
}