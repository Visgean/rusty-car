use linux_embedded_hal::i2cdev::linux::LinuxI2CError;
use crate::motors::MotorControl;
use std::thread::sleep;
use std::time::Duration;
use gilrs::{Gilrs, Button, Event, Axis};
use gilrs::ev::state::GamepadState;

mod motors;


fn controller_loop(mut motor: MotorControl) {
    let mut gilrs = Gilrs::new().unwrap();

    // show connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut active_gamepad = None;

    loop {
        // loop over events as they happen on the gamepad
        // fixme: maybe it would be better to run 25hz loop and simply query the state?
        // I am not sure if the state of the controller depends on the event which i am processing
        // which could cause the "Controlling lag" that I am seeing
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            active_gamepad = Some(id);

            // You can also use cached gamepad state
            if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
                let max_speed = motors::MAX_WHEEL_SPEED as f32;

                // gamepad.value is float between -1 and 1
                let right_side = (gamepad.value(Axis::RightStickY) * max_speed) as i32;
                let left_side = (gamepad.value(Axis::LeftStickY) * max_speed) as i32;

                // it seems the motors are reversed and this is an easy fix for now
                motor.move_all_wheels(-left_side, -left_side, -right_side, -right_side).unwrap();
            }
        }
    }
}


fn main() {
    let mut motor = motors::MotorControl::new().unwrap();
    controller_loop(motor)
}