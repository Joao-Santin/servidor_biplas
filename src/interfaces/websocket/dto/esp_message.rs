use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum EspMessage {

    #[serde(rename = "identify")]
    Identify {
        id: String,
        sector: String,
        controller: String,
        ip: String,
        mac: String,
        timestamp: u64,
        payload: IdentifyPayload,
    },

    #[serde(rename = "heartbeat")]
    Heartbeat {
        id: String,
        sector: String,
        controller: String,
        ip: String,
        mac: String,
        timestamp: u64,
        payload: HeartbeatPayload,
    },

    #[serde(rename = "signal_received")]
    SignalReceived {
        id: String,
        sector: String,
        controller: String,
        ip: String,
        mac: String,
        timestamp: u64,
        payload: SignalPayload,
    },
}
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename = "payload")]
pub struct HeartbeatPayload {
    pub wifi_rssi: i32,
    pub free_heap: u32,
}
#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "payload")]
pub struct IdentifyPayload{
    pub ip: String,
    pub mac: String,
}
#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "payload")]
pub struct SignalPayload {
    pub signal: String,
    pub pin: u8,
    pub value: bool,
}
