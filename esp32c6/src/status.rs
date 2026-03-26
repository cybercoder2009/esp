pub struct Status {
    pub temp: f32,
    pub wifi_ssid: Option<[u8; 32]>,
    pub wifi_pass: Option<[u8; 64]>,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            temp: 0.0,
            wifi_ssid: None,
            wifi_pass: None,
        }
    }
}
