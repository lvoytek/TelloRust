mod tello_controller;

use std::{thread, time};

fn main() {
    tello_controller::activate_drone();
    tello_controller::takeoff();

    thread::sleep(time::Duration::from_secs(3));

    tello_controller::land();
}
