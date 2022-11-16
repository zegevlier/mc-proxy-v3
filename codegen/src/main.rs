#![allow(dead_code)]
use serde::Deserialize;
use serde_json::{ser::Formatter, Value};

use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Deserialize)]
struct Packet {
    class: String,
    direction: Direction,
    from_client: bool,
    from_server: bool,
    id: i32,
    instructions: Vec<Instruction>,
    state: State,
}

#[derive(Debug, Deserialize)]
enum Direction {
    #[serde(rename = "SERVERBOUND")]
    Serverbound,
    #[serde(rename = "CLIENTBOUND")]
    Clientbound,
}

#[derive(Debug, Deserialize)]
enum State {
    #[serde(rename = "HANDSHAKING")]
    Handshaking,
    #[serde(rename = "STATUS")]
    Status,
    #[serde(rename = "LOGIN")]
    Login,
    #[serde(rename = "PLAY")]
    Play,
}

#[derive(Debug, Deserialize)]
struct Instruction {
    field: Option<String>,
    condition: Option<String>,
    operation: Operation,
    #[serde(rename = "type", default)]
    instruction_type: OptionalInstructionType,
    instructions: Option<Vec<Instruction>>,
}

#[derive(Debug, Deserialize)]
enum Operation {
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "store")]
    Store,
    #[serde(rename = "if")]
    If,
    #[serde(rename = "else")]
    Else,
    #[serde(rename = "loop")]
    Loop,
    #[serde(rename = "interfacecall")]
    InterfaceCall,
    #[serde(rename = "increment")]
    Increment,
    #[serde(rename = "putfield")]
    PutField,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
enum InstructionType {
    #[serde(rename = "varint")]
    Varint,
    #[serde(rename = "varlong")]
    Varlong,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "short")]
    Short,
    #[serde(rename = "chatcomponent")]
    ChatComponent,
    #[serde(rename = "byte[]")]
    ByteArray,
    #[serde(rename = "uuid")]
    Uuid,
    #[serde(rename = "Iterator")]
    Iterator,
    #[serde(rename = "identifier")]
    Identifier,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "byte")]
    Byte,
    #[serde(rename = "position")]
    Position,
    #[serde(rename = "nbtcompound")]
    NbtCompound,
    #[serde(rename = "enum")]
    Enum,
    #[serde(rename = "interface")]
    Interface,
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "itemstack")]
    ItemStack,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "long[]")]
    LongArray,
    #[serde(rename = "Object")]
    Object,
    #[serde(rename = "metadata")]
    Metadata,
    #[serde(rename = "varint[]")]
    VarintArray,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(untagged)]
enum OptionalInstructionType {
    Some(InstructionType),
    Other(String),
    None,
}

impl Display for OptionalInstructionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionalInstructionType::Some(instruction_type) => write!(f, "{:?}", instruction_type),
            OptionalInstructionType::Other(other) => write!(f, "{}", other),
            OptionalInstructionType::None => write!(f, "None"),
        }
    }
}

impl Default for OptionalInstructionType {
    fn default() -> Self {
        OptionalInstructionType::None
    }
}

#[derive(Debug)]
struct Class {
    name: String,
    obfuscated_name: String,
    fields: HashMap<String, String>,
}

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
    let mappings_u = std::fs::read_to_string("mappings.txt").unwrap();

    let mut mappings: HashMap<String, Class> = HashMap::new();
    let mut current_class = Class {
        name: String::new(),
        obfuscated_name: String::new(),
        fields: HashMap::new(),
    };
    for line in mappings_u
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        if line.starts_with("    ") {
            let mut split = line.split(" -> ");
            let first_part = split.next().unwrap();
            if first_part
                .strip_prefix("    ")
                .unwrap()
                .split(' ')
                .next()
                .unwrap()
                .contains(':')
            {
                continue;
            }
            let real = first_part.split(' ').last().unwrap();
            let obfus = split.next().unwrap();
            current_class
                .fields
                .insert(obfus.to_string(), real.to_string());
        } else {
            if !current_class.name.is_empty() {
                mappings.insert(current_class.obfuscated_name.clone(), current_class);
            }
            let mut split = line.split(" -> ");
            let real = split.next().unwrap();
            let obfus = split.next().unwrap().strip_suffix(':').unwrap();
            current_class = Class {
                name: real.to_string(),
                obfuscated_name: obfus.to_string(),
                fields: HashMap::new(),
            };
        }
    }

    let packets = packet_data
        .as_object()
        .unwrap()
        .get("packet")
        .unwrap()
        .as_object()
        .unwrap();
    for (_packet_id, packet) in packets.iter() {
        let packet: Packet = serde_json::from_value(packet.to_owned()).unwrap();
        let class = packet.class;
        let class = mappings
            .get(class.strip_suffix(".class").unwrap())
            .unwrap()
            .name
            .split('.')
            .last()
            .unwrap();

        for instruction in packet.instructions {
            print_instruction(class, &instruction);
        }
    }
}

fn print_instruction(prefix: &str, instruction: &Instruction) {
    match &instruction.instruction_type {
        OptionalInstructionType::Some(instruction_type) => {
            println!("{} -> {:?}", prefix, instruction_type);
        }
        OptionalInstructionType::Other(thing) => {
            println!("{} -> {}", prefix, thing);
        }
        OptionalInstructionType::None => {
            println!("{} -> None", prefix);
        }
    }
    if let Some(instructions) = instruction.instructions.as_ref() {
        for instruction in instructions {
            print_instruction(
                &format!("{}:{:?}", prefix, instruction.instruction_type),
                instruction,
            );
        }
    }
}
