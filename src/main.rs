mod message;
mod server;
mod socket;

use server::SadpServer;
use socket::SadpSocket;

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let socket = SadpSocket::new();
    socket.inquiry();
    let server = SadpServer::new(&socket);
    server.run(|msg| {
        if msg.analog_channel_num == arg.parse::<u32>().unwrap() {
            println!("IPv4 Address:\t\t{}", msg.ipv4_address);
            println!("Software Version:\t{}", msg.software_version);
            true
        } else {
            false
        }
    });
}
