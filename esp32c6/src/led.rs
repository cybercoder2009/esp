use esp_hal::{
    delay::Delay,
    gpio::OutputPin,
    rmt::{PulseCode, TxChannelCreator},
    Blocking,
};
use esp_hal_smartled::SmartLedsAdapter;
use smart_leds::{brightness, SmartLedsWrite, RGB8};

pub struct Led<'a, const BUFFER_SIZE: usize> {
    led: SmartLedsAdapter<'a, BUFFER_SIZE>,
    delay: Delay,
}

impl<'a, const BUFFER_SIZE: usize> Led<'a, BUFFER_SIZE> {
    pub fn new<P>(
        channel: impl TxChannelCreator<'a, Blocking>,
        pin: P,
        buffer: &'a mut [PulseCode; BUFFER_SIZE],
    ) -> Self
    where
        P: OutputPin + 'a,
    {
        let led = SmartLedsAdapter::new(channel, pin, buffer);
        let delay = Delay::new();
        Self { led, delay }
    }

    pub fn blink(&mut self, color: RGB8, time_ms: u32) {
        // RGB -> GRB
        let corrected_color = RGB8::new(color.g, color.r, color.b);
        self.led
            .write(brightness([corrected_color].iter().cloned(), 25))
            .unwrap();
        self.delay.delay_millis(time_ms);
        self.led
            .write([RGB8::new(0, 0, 0)].iter().cloned())
            .unwrap();
        self.delay.delay_millis(time_ms);
    }

    pub fn blink_red(&mut self, time_ms: u32) {
        self.blink(RGB8::new(255, 0, 0), time_ms);
    }

    pub fn blink_green(&mut self, time_ms: u32) {
        self.blink(RGB8::new(0, 255, 0), time_ms);
    }

    pub fn blink_blue(&mut self, time_ms: u32) {
        self.blink(RGB8::new(0, 0, 255), time_ms);
    }
}

#[macro_export]
macro_rules! init_led {
    ($p:expr) => {{
        use esp_hal::{rmt::Rmt, time::Rate};
        let rmt = Rmt::new($p.RMT, Rate::from_mhz(80)).expect("Failed to init RMT");
        static mut BUFFER: [esp_hal::rmt::PulseCode; 25] =
            [esp_hal::rmt::PulseCode::end_marker(); 25];
        $crate::led::Led::new(rmt.channel0, $p.GPIO8, unsafe { &mut BUFFER })
    }};
}
