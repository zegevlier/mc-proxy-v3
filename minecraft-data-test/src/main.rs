use convert_case::{Case, Casing};
use minecraft_data_rs::api::{versions_by_minecraft_version, Api};

fn main() -> anyhow::Result<()> {
    println!("Getting api...");
    let api = Api::new(
        versions_by_minecraft_version()?
            .get("1.19.2")
            .unwrap()
            .clone(),
    );
    println!("Getting protocol...");
    let protocol = api.protocols.get_protocol()?;
    for packet in protocol.handshaking.to_server.types.iter() {
        println!("Packet: {}", packet.name.to_case(Case::UpperCamel));
        println!("Data: {:?}", packet.data);
        match packet.data.to_owned() {
            minecraft_data_rs::models::protocol::PacketDataType::Built { name, value } => {
                println!("Built: {} {:?}", name, value);
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}
