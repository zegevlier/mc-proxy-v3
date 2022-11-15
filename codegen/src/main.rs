use serde_json::Value;

use std::collections::HashMap;

fn main() {
    // read "burger_out.json" and parse it into a `Value`
    let json = std::fs::read_to_string("burger_out.json").unwrap();
    let value: Value = serde_json::from_str(&json).unwrap();
    let packet_data = value
        .as_array()
        .unwrap()
        .get(0)
        .unwrap()
        .as_object()
        .unwrap()
        .get("packets")
        .unwrap();
    let mappings = std::fs::read_to_string("mappings.txt").unwrap();
    let mappings = mappings
        .lines()
        .filter(|l| !l.starts_with(' ') && !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.split(" -> ").collect::<Vec<&str>>())
        .map(|l| (l[1].strip_suffix(':').unwrap(), l[0]))
        .collect::<HashMap<&str, &str>>();

    let packets = packet_data
        .as_object()
        .unwrap()
        .get("packet")
        .unwrap()
        .as_object()
        .unwrap();
    for packet in packets.keys() {
        let p = packets.get(packet).unwrap();
        let class = p
            .as_object()
            .unwrap()
            .get("class")
            .unwrap()
            .as_str()
            .unwrap();

        let class = mappings
            .get(class.strip_suffix(".class").unwrap())
            .unwrap()
            .split('.')
            .last()
            .unwrap();
        println!("{} -> {}", packet, class);
    }
}
