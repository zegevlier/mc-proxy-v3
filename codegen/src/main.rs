#![allow(dead_code)]
use serde::Deserialize;
use serde_json::Value;

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
    var: Option<String>,
    value: Option<String>,
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
    fields: HashMap<String, Field>,
    methods: HashMap<String, Method>,
}

#[derive(Debug)]
struct Field {
    name: String,
    field_type: String,
    obfuscated_name: String,
}

#[derive(Debug)]
struct Method {
    name: String,
    obfuscated_name: String,
    parameters: String,
    return_type: String,
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

    let mut classes = Vec::new();

    let mapping = proguard::ProguardMapping::new(mappings_u.as_bytes());
    for record in mapping.iter() {
        match record {
            Ok(r) => match r {
                proguard::ProguardRecord::Header { key: _, value: _ } => {}
                proguard::ProguardRecord::Class {
                    original,
                    obfuscated,
                } => {
                    // println!("\t{} -> {}", original, obfuscated);
                    classes.push(Class {
                        name: original.to_string(),
                        obfuscated_name: obfuscated.to_string(),
                        fields: HashMap::new(),
                        methods: HashMap::new(),
                    });
                }
                proguard::ProguardRecord::Field {
                    ty,
                    original,
                    obfuscated,
                } => {
                    // println!("\t\t{} {} -> {}", ty, original, obfuscated);
                    let idx = classes.len() - 1;
                    let class = classes.get_mut(idx).unwrap();
                    class.fields.insert(
                        obfuscated.to_string(),
                        Field {
                            name: original.to_string(),
                            field_type: ty.to_string(),
                            obfuscated_name: obfuscated.to_string(),
                        },
                    );
                }
                proguard::ProguardRecord::Method {
                    ty,
                    original,
                    obfuscated,
                    arguments,
                    original_class: _,
                    line_mapping: _,
                } => {
                    // println!(
                    //     "\t\t{} {} {} {:?} -> {}",
                    //     ty, original, arguments, original_class, obfuscated
                    // );

                    let idx = classes.len() - 1;
                    let class = classes.get_mut(idx).unwrap();
                    class.methods.insert(
                        obfuscated.to_string(),
                        Method {
                            name: original.to_string(),
                            obfuscated_name: obfuscated.to_string(),
                            parameters: arguments.to_string(),
                            return_type: ty.to_string(),
                        },
                    );
                }
            },
            Err(_) => todo!(),
        }
    }

    let mut mappings = HashMap::new();
    for class in classes {
        mappings.insert(class.obfuscated_name.clone(), class);
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
        let class_unmapped = packet.class.strip_suffix(".class").unwrap();
        let class = mappings.get(class_unmapped).unwrap();
        gen_packet_code(&packet, class);

        // println!("{}", class.name);

        // for instruction in packet.instructions {
        //     // print_instruction(class, &instruction);
        //     if let Some(instruction_field) = instruction.field {
        //         if let Some(remapped_instruction) = class.fields.get(&instruction_field) {
        //             println!("\t{:?}", remapped_instruction);
        //         } else {
        //             println!("\t{:?}", instruction_field);
        //         }
        //     }
        // }
    }
}

fn gen_packet_code(packet: &Packet, class: &Class) {
    println!("#[derive(Packet)]");
    println!("pub struct {} {{", class.name.split('.').last().unwrap());

    print_instruction_code(&packet.instructions, class, 1);

    println!("}}");
}

fn print_instruction_code(instructions: &Vec<Instruction>, class: &Class, tab_count: usize) {
    for instruction in instructions {
        match instruction.operation {
            Operation::Write => {
                match class.fields.get(instruction.field.as_ref().unwrap()) {
                    Some(field) => {
                        println!(
                            "{}pub {}: {},",
                            "\t".repeat(tab_count),
                            field.name,
                            instruction.instruction_type
                        );
                    }
                    None => {
                        println!(
                            "{}//TODO {}: {},",
                            "\t".repeat(tab_count),
                            instruction.field.as_ref().unwrap(),
                            instruction.instruction_type
                        );
                    }
                };
            }
            Operation::Store => {
                match class.fields.get(instruction.var.as_ref().unwrap()) {
                    Some(field) => {
                        println!(
                            "{}let {}: {} = {};",
                            "\t".repeat(tab_count),
                            field.name,
                            instruction.instruction_type,
                            instruction.value.as_ref().unwrap()
                        );
                    }
                    None => {
                        println!(
                            "{}//TODO let {}: {} = {};",
                            "\t".repeat(tab_count),
                            instruction.var.as_ref().unwrap(),
                            instruction.instruction_type,
                            instruction.value.as_ref().unwrap()
                        );
                    }
                };
            }
            Operation::If => {
                println!(
                    "{}if {} {{",
                    "\t".repeat(tab_count),
                    instruction.condition.as_ref().unwrap()
                );
                print_instruction_code(
                    instruction.instructions.as_ref().unwrap(),
                    class,
                    tab_count + 1,
                );
                println!("{}}}", "\t".repeat(tab_count));
            }
            Operation::Else => {
                println!("{}else {{", "\t".repeat(tab_count));
                print_instruction_code(
                    instruction.instructions.as_ref().unwrap(),
                    class,
                    tab_count + 1,
                );
                println!("{}}}", "\t".repeat(tab_count));
            }
            Operation::Loop => {
                println!(
                    "{}while ({}) {{",
                    "\t".repeat(tab_count),
                    instruction.condition.as_ref().unwrap()
                );
                print_instruction_code(
                    instruction.instructions.as_ref().unwrap(),
                    class,
                    tab_count + 1,
                );
                println!("{}}}", "\t".repeat(tab_count));
            }
            Operation::InterfaceCall => todo!(),
            Operation::Increment => todo!(),
            Operation::PutField => todo!(),
        }
    }
}
