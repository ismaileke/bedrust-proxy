use std::sync::Arc;
use bedrock_client::protocol::raknet::packet_ids::PacketType;
use bedrock_client::protocol::raknet::unconnected_ping::UnconnectedPing;
use bedrock_client::protocol::raknet::unconnected_pong::UnconnectedPong;
use binary_utils::binary::Stream;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {

    let socket = UdpSocket::bind("0.0.0.0:19132").await.expect("Failed to bind socket");
    let socket = Arc::new(socket);

    println!("RakNet Listener is running on port 19132...");

    let mut buf = [0u8; 2048];

    loop {
        let (len, src_addr) = socket.recv_from(&mut buf).await.expect("Failed to receive data");

        let stream = Stream::new(Vec::from(&buf[..len]), 0);

        let packet_type = PacketType::from_byte(buf[0]);

        match packet_type {
            PacketType::UnconnectedPing => {
                let server_guid: u64 = 1435234;
                let unconnected_ping = UnconnectedPing::decode(Vec::from(stream.get_buffer()));
                let ping_time = unconnected_ping.ping_time;
                let unconnected_pong = UnconnectedPong::create(
                    ping_time,
                    server_guid,
                    build_motd(
                        "Bedrust Proxy",
                        "§7High-performance Minecraft Bedrock proxy written in Rust",
                        898,
                        "1.21.132",
                        5,
                        100,
                        server_guid,
                        19132
                    )).encode();
                socket.send_to(&unconnected_pong, src_addr).await.expect("Failed to send data");
            },
            PacketType::OpenConnReq1 => {

            },
            PacketType::OpenConnReq2 => {

            },
            PacketType::ConnReq => {

            },
            _ => {}
        }
    }


}

pub fn build_motd(
    server_name: &str,
    desc: &str,
    protocol: u32,
    version: &str,
    online: u32,
    max: u32,
    guid: u64,
    ipv4_port: u16,
) -> String {
    format!(
        "MCPE;{};{};{};{};{};{};{};Survival;1;{};{};",
        server_name,
        protocol,
        version,
        online,
        max,
        guid,
        desc,
        ipv4_port,
        ipv4_port + 1
    )
}