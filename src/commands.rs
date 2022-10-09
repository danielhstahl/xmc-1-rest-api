use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, PartialEq)]
pub struct CommandName {
    #[serde(rename = "@value")]
    value: String,
    #[serde(rename = "@ack")]
    ack: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandTypes {
    PowerOn(CommandName),
    Volume(CommandName),
    SetVolume(CommandName),
    Hdmi1(CommandName),
}

#[derive(Debug, Serialize)]
#[serde(rename = "emotivaControl")]
pub struct EmotivaControl {
    command: CommandTypes,
}

impl EmotivaControl {
    pub fn power_on() -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::PowerOn(CommandName {
                value: "0".to_string(),
                ack: "yes",
            }),
        }
    }
    pub fn volume_up(increment: i32) -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::Volume(CommandName {
                value: increment.to_string(),
                ack: "yes",
            }),
        }
    }
    pub fn volume_down(increment: i32) -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::Volume(CommandName {
                value: (-increment).to_string(),
                ack: "yes",
            }),
        }
    }
    //-96 to 11
    pub fn set_volume(value: i32) -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::SetVolume(CommandName {
                value: value.to_string(),
                ack: "yes",
            }),
        }
    }
    pub fn set_hdmi_1() -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::Hdmi1(CommandName {
                value: "0".to_string(),
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
pub struct Control {
    pub version: f32,
    pub control_port: i32,
    pub notify_port: i32,
    pub info_port: i32,
    pub setup_port_t_c_p: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "emotivaTransponder")]
#[serde(rename_all = "camelCase")]
pub struct EmotivaTransponder {
    pub model: String,
    pub name: String,
    pub control: Control,
}
