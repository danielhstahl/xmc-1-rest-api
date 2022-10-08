use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//use serde_xml_rs::{from_str, to_string};

#[derive(Debug, Serialize, PartialEq)]
//#[serde(rename = "#text")]
pub struct CommandName {
    #[serde(rename = "@value")]
    value: String,
    #[serde(rename = "@ack")]
    ack: &'static str,
}

#[derive(Debug, Serialize)]
//#[serde(rename = "emotivaControl")]
#[serde(rename_all = "snake_case")]
//#[serde(untagged)]
//#[serde(tag = "tag")]
pub enum CommandTypes {
    //#[serde(rename = "#text")]
    //command: CommandName, //HashMap<String, CommandName>,
    PowerOn(CommandName),
    Volume(CommandName),
}

#[derive(Debug, Serialize)]
#[serde(rename = "emotivaControl")]
pub struct EmotivaControl {
    //#[serde(rename = "#text")]
    //command: CommandName, //HashMap<String, CommandName>,
    //PowerOn(CommandName),
    //#[serde(rename = "#text")]
    command: CommandTypes,
}

/*
#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "emotivaControl")]
pub struct EmotivaControl {
    #[serde(rename = "$value")]
    items: Vec<CommandName>,
}*/
/*
impl EmotivaControl {
    pub fn new(command: String) -> EmotivaControl {
        EmotivaControl {
            command_name: CommandName {
                value: command,
                ack: "yes",
            },
        }
    }
}*/

impl EmotivaControl {
    pub fn new(command: String) -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::PowerOn(CommandName {
                value: command,
                ack: "yes",
            }),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename = "emotivaPing")]
#[serde(rename_all = "camelCase")]
pub struct EmotivaPing {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Control {
    version: f32,
    control_port: i32,
    notify_port: i32,
    info_port: i32,
    setup_port_t_c_p: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "emotivaTransponder")]
#[serde(rename_all = "camelCase")]
pub struct EmotivaTransponder {
    model: String,
    name: String,
    control: Control,
}
