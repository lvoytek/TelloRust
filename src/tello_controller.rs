use std::net::UdpSocket;

pub fn activate_drone() {
 send_command("command");
}

pub fn takeoff() {
  send_command("takeoff");
}

pub fn land() {
  send_command("land");
}

pub fn activate_video() {
  send_command("streamon");
}

pub fn deactivate_video() {
  send_command("streamoff");
}

pub fn kill() {
  send_command("emergency");
}

/// Attempt to fly the drone upward the given distance in cm between 20 and 500
pub fn fly_up(distance: u16) {
  send_directional_movement_command("up", distance);
}

/// Attempt to fly the drone downward the given distance in cm between 20 and
/// 500
pub fn fly_down(distance: u16) {
  send_directional_movement_command("down", distance);
}

/// Attempt to fly the drone leftward the given distance in cm between 20 and
/// 500
pub fn fly_left(distance: u16) {
  send_directional_movement_command("left", distance);
}

/// Attempt to fly the drone rightward the given distance in cm between 20 and 500
pub fn fly_right(distance: u16) {
  send_directional_movement_command("right", distance);
}

/// Attempt to fly the drone forward the given distance in cm between 20 and
/// 500
pub fn fly_forward(distance: u16) {
  send_directional_movement_command("forward", distance);
}

/// Attempt to fly the drone backward the given distance in cm between 20 and
/// 500
pub fn fly_backward(distance: u16) {
  send_directional_movement_command("back", distance);
}

/// Attempt to rotate the drone clockwise a given number of degrees between 1
/// and 3600
pub fn rotate_cw(degree_amount: u16) {
  send_rotational_movement_command("cw", degree_amount);
}

/// Attempt to rotate the drone counter-clockwise a given number of degrees
/// between 1 and 3600
pub fn rotate_ccw(degree_amount: u16) {
  send_rotational_movement_command("ccw", degree_amount);
}

/// Attempt to fly the drone in the given direction over a given number of cm
/// Movement is limited to the range of 20-500cm
fn send_directional_movement_command(direction: &str, distance: u16) {
  let mut new_distance = distance;

  if new_distance < 20 {
    new_distance = 20;
  }

  if new_distance > 500 {
    new_distance = 500;
  }

  send_command(format!("{} {}", direction, new_distance).as_str());
}

/// Attempt to rotate the drone in a certain direction a certain number of
/// degrees. Rotation is limited between 1 and 3600 degrees
fn send_rotational_movement_command(direction: &str, amount: u16) {
  let mut new_amount = amount;

  if new_amount < 1 {
    new_amount = 1;
  }

  if new_amount > 3600 {
    new_amount = 3600;
  }

  send_command(format!("{} {}", direction, new_amount).as_str());
}

fn send_command(command: &str) {
  println!("{}", command);
  let socket = UdpSocket::bind("[::]:2000").expect("Port unavailable");
  socket.connect("192.168.10.1:8889").expect("Unable to connect to drone");
  socket.send_to(command.as_bytes(), "192.168.10.1:8889").expect("Unable to send command");
}
