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

fn send_command(command: &str) {
  println!("{}", command);
  let socket = UdpSocket::bind("[::]:2000").expect("Port unavailable");
  socket.connect("192.168.10.1:8889").expect("Unable to connect to drone");
  socket.send_to(command.as_bytes(), "192.168.10.1:8889").expect("Unable to send command");
}
