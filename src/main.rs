mod tello_controller;

use std::{thread, time};

fn main() {
    tello_controller::activate_drone();
    tello_controller::takeoff();
    tello_controller::fly_up(50);
    tello_controller::rotate_cw(360);

    thread::sleep(time::Duration::from_secs(3));

    tello_controller::land();
}
