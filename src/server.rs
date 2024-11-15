use crate::{message::ProbeMatch, socket::SadpSocket};

pub struct SadpServer {
    socket: SadpSocket,
}

impl SadpServer {
    pub fn new(socket: &SadpSocket) -> Self {
        Self {
            socket: socket.clone(),
        }
    }

    pub fn run(&self, f: impl Fn(ProbeMatch) -> bool) {
        let mut buf = [0u8; 65536];
        loop {
            let (size, _) = self.socket.recv_from(&mut buf).unwrap();
            let msg = String::from_utf8(buf[..size].to_vec()).unwrap();
            if let Ok(msg) = serde_xml_rs::from_str::<ProbeMatch>(&msg) {
                if f(msg) {
                    break;
                }
            }
        }
    }
}
