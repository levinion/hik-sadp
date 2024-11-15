use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProbeMatch {
    #[serde(rename = "Uuid")]
    pub uuid: String,
    #[serde(rename = "Types")]
    pub types: String,
    #[serde(rename = "DeviceType")]
    pub device_type: u32,
    #[serde(rename = "IPv4Address")]
    pub ipv4_address: String,
    #[serde(rename = "AnalogChannelNum")]
    pub analog_channel_num: u32,
    #[serde(rename = "SoftwareVersion")]
    pub software_version: String,
    #[serde(rename = "Activated")]
    pub activated: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Probe {
    #[serde(rename = "Uuid")]
    pub uuid: String,
    #[serde(rename = "Types")]
    pub types: String,
}
