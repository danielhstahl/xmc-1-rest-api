use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CommandName {
    #[serde(rename = "@value")]
    #[serde(skip_deserializing)]
    value: String,
    #[serde(rename = "@ack")]
    #[serde(skip_deserializing)]
    ack: String,
    #[serde(rename = "@status")]
    #[serde(skip_serializing)]
    status: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Response<T> {
    #[serde(rename = "@value")]
    value: T,
    #[serde(rename = "@status")]
    status: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandTypes {
    PowerOn(CommandName),
    Standby(CommandName),
    Volume(CommandName),
    SetVolume(CommandName),
    Hdmi1(CommandName),
    Hdmi2(CommandName),
    Hdmi3(CommandName),
    Hdmi4(CommandName),
    Hdmi5(CommandName),
    Hdmi6(CommandName),
    Hdmi7(CommandName),
    Hdmi8(CommandName),
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
                ack: "yes".to_string(),
                status: "anything".to_string(),
            }),
        }
    }
    pub fn standby() -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::Standby(CommandName {
                value: "0".to_string(),
                ack: "yes".to_string(),
                status: "anything".to_string(),
            }),
        }
    }
    pub fn volume_up(increment: i32) -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::Volume(CommandName {
                value: increment.to_string(),
                ack: "yes".to_string(),
                status: "anything".to_string(),
            }),
        }
    }
    pub fn volume_down(increment: i32) -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::Volume(CommandName {
                value: (-increment).to_string(),
                ack: "yes".to_string(),
                status: "anything".to_string(),
            }),
        }
    }
    //-96 to 11
    pub fn set_volume(value: i32) -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::SetVolume(CommandName {
                value: value.to_string(),
                ack: "yes".to_string(),
                status: "anything".to_string(),
            }),
        }
    }
    pub fn set_hdmi_1() -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::Hdmi1(CommandName {
                value: "0".to_string(),
                ack: "yes".to_string(),
                status: "anything".to_string(),
            }),
        }
    }
    pub fn set_hdmi_2() -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::Hdmi2(CommandName {
                value: "0".to_string(),
                ack: "yes".to_string(),
                status: "anything".to_string(),
            }),
        }
    }
    pub fn set_hdmi_3() -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::Hdmi3(CommandName {
                value: "0".to_string(),
                ack: "yes".to_string(),
                status: "anything".to_string(),
            }),
        }
    }
    pub fn set_hdmi_4() -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::Hdmi4(CommandName {
                value: "0".to_string(),
                ack: "yes".to_string(),
                status: "anything".to_string(),
            }),
        }
    }
    pub fn set_hdmi_5() -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::Hdmi5(CommandName {
                value: "0".to_string(),
                ack: "yes".to_string(),
                status: "anything".to_string(),
            }),
        }
    }
    pub fn set_hdmi_6() -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::Hdmi6(CommandName {
                value: "0".to_string(),
                ack: "yes".to_string(),
                status: "anything".to_string(),
            }),
        }
    }
    pub fn set_hdmi_7() -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::Hdmi7(CommandName {
                value: "0".to_string(),
                ack: "yes".to_string(),
                status: "anything".to_string(),
            }),
        }
    }
    pub fn set_hdmi_8() -> EmotivaControl {
        EmotivaControl {
            command: CommandTypes::Hdmi8(CommandName {
                value: "0".to_string(),
                ack: "yes".to_string(),
                status: "anything".to_string(),
            }),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename = "emotivaPing")]
pub struct EmotivaPing {}

#[derive(Debug, Serialize)]
pub struct EmptyStruct {}

#[derive(Debug, Serialize)]
#[serde(rename = "emotivaUpdate")]
#[serde(rename_all = "snake_case")]
pub struct RequestInfo {
    power: EmptyStruct,
    source: EmptyStruct,
    volume: EmptyStruct,
    mode: EmptyStruct,
    audio_bits: EmptyStruct,
    audio_bitstream: EmptyStruct,
    video_format: EmptyStruct,
}

impl RequestInfo {
    pub fn new() -> Self {
        RequestInfo {
            power: EmptyStruct {},
            source: EmptyStruct {},
            volume: EmptyStruct {},
            mode: EmptyStruct {},
            audio_bits: EmptyStruct {},
            audio_bitstream: EmptyStruct {},
            video_format: EmptyStruct {},
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename = "emotivaUpdate")]
#[serde(rename_all = "snake_case")]
pub struct GetInfo {
    power: Response<String>,
    source: Response<String>,
    volume: Response<f32>,
    mode: Response<String>,
    audio_bits: Response<String>,
    audio_bitstream: Response<String>,
    video_format: Response<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    power: String,
    source: String,
    volume: f32,
    mode: String,
    audio_bits: String,
    audio_bitstream: String,
    video_format: String,
}

impl Info {
    pub fn from(info: GetInfo) -> Self {
        Info {
            power: info.power.value,
            source: info.source.value,
            volume: info.volume.value,
            audio_bits: info.audio_bits.value,
            audio_bitstream: info.audio_bitstream.value,
            mode: info.mode.value,
            video_format: info.video_format.value,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Control {
    pub version: f32,
    pub control_port: u16,
    pub notify_port: u16,
    pub info_port: u16,
    pub setup_port_t_c_p: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "emotivaTransponder")]
#[serde(rename_all = "camelCase")]
pub struct EmotivaTransponder {
    pub model: String,
    pub name: String,
    pub control: Control,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmotivaAck {
    #[serde(flatten)]
    any: CommandTypes,
}

#[cfg(test)]
mod tests {

    use super::{EmotivaAck, EmotivaControl};
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;

    #[test]
    fn test_serialization() {
        let result = EmotivaControl::power_on();
        let _ = to_string(&result).unwrap();
    }

    #[test]
    fn test_deserialization() {
        let message = r#"<?xml version="1.0"?><emotivaAck><power_on status="ack"/></emotivaAck>"#;
        let _: EmotivaAck = from_str(&message).unwrap();
    }
}
