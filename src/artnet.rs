use artnet_protocol::{ArtCommand, Output, PaddedData, PortAddress};
use crossbeam_channel::{unbounded, Sender};
use log::{error, info};
use std::net::{ToSocketAddrs, UdpSocket};

pub fn start() -> Sender<Vec<u8>> {
    let (s, r) = unbounded::<Vec<u8>>();

    std::thread::Builder::new()
        .name("gamepad_artnet:tx".to_owned())
        .spawn(move || {
            let socket = UdpSocket::bind(("0.0.0.0", 6001)).unwrap();
            match socket.set_broadcast(true) {
                Ok(_) => info!("Activated sending to broadcast"),
                Err(e) => info!("Could not activate sending to broadcast: {}", e),
            }
            match socket.set_nonblocking(true) {
                Ok(_) => info!("Activated non-blocking mode"),
                Err(e) => info!("Could not activate non-blocking mode: {}", e),
            };
            let addr = (crate::OPTS.artnet_host.as_str(), 6454)
                .to_socket_addrs()
                .ok()
                .and_then(|mut addr| addr.next())
                .expect("Artnet host is invalid!");
            let port_address = PortAddress::try_from(crate::OPTS.artnet_universe)
                .expect("Artnet universe is invalid!");

            for data in r.iter() {
                let output = Output {
                    data: PaddedData::from(data),
                    port_address,
                    ..Default::default()
                };

                let command = ArtCommand::Output(output);
                let _ = command
                    .write_to_buffer()
                    .map_err(|e| error!("Could not convert command into buffer: {:?}", e))
                    .map(|bytes| {
                        socket
                            .send_to(&bytes, addr)
                            .map_err(|e| error!("Could not send data: {:?}", e))
                    });
            }
        })
        .expect("Could not start artnet thread");

    s
}
