use std::{
    net::{Ipv4Addr, SocketAddr},
    ops::Deref,
    sync::Arc,
};

use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use uuid::Uuid;

use crate::message::Probe;

#[derive(Clone)]
pub struct SadpSocket {
    socket: Arc<Socket>,
}

impl Deref for SadpSocket {
    type Target = Arc<Socket>;
    fn deref(&self) -> &Self::Target {
        &self.socket
    }
}

impl SadpSocket {
    pub fn new() -> Self {
        let multiaddr = "239.255.255.250".parse::<Ipv4Addr>().unwrap();
        let interface = "10.20.138.200".parse::<Ipv4Addr>().unwrap();
        let local_addr = "0.0.0.0:37020".parse::<SocketAddr>().unwrap();
        let domain = Domain::ipv4();
        let socket = Socket::new(domain, Type::dgram(), Some(Protocol::udp())).unwrap();
        socket.set_reuse_address(true).unwrap();
        #[cfg(not(target_os = "windows"))]
        socket.set_reuse_port(true).unwrap();
        socket.set_multicast_loop_v4(false).unwrap();
        socket.set_ttl(2).unwrap();
        socket.join_multicast_v4(&multiaddr, &interface).unwrap();
        socket.bind(&socket2::SockAddr::from(local_addr)).unwrap();
        let socket = Arc::new(socket);
        Self { socket }
    }

    pub fn send(&self, message: &[u8]) {
        let remote = "239.255.255.250:37020".parse::<SocketAddr>().unwrap();
        self.send_to(message, &SockAddr::from(remote)).unwrap();
    }

    pub fn inquiry(&self) -> String {
        let probe = Probe {
            uuid: Uuid::new_v4().to_string(),
            types: "inquiry".to_string(),
        };
        self.send(serde_xml_rs::to_string(&probe).unwrap().as_bytes());
        probe.uuid
    }
}
