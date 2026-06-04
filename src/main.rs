use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::peripherals::Peripherals;
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_esp32_rmt_driver::Ws2812Esp32RmtDriver;

fn main() {
    // Required to apply patches to the ESP-IDF runtime at link time.
    // See: https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP-IDF logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("ESP32-C3 RGB LED blink starting!");

    // Take ownership of all board peripherals
    let peripherals = Peripherals::take().unwrap();

    // GPIO8 is the on-board RGB LED (WS2812B) on most ESP32-C3 DevKit boards
    let led_pin = peripherals.pins.gpio8;

    // Use RMT channel 0 to generate the WS2812B signal
    let rmt_channel = peripherals.rmt.channel0;

    let mut driver = Ws2812Esp32RmtDriver::new(rmt_channel, led_pin)
        .expect("Failed to initialize WS2812 RMT driver");

    // Colors to cycle through (each shown for 500 ms)
    let colors: [RGB8; 4] = [
        RGB8::new(255, 0, 0), // Red
        RGB8::new(0, 255, 0), // Green
        RGB8::new(0, 0, 255), // Blue
        RGB8::new(0, 0, 0),   // Off  ← creates the visible "blink" effect
    ];

    let mut color_index = 0usize;

    loop {
        let color = colors[color_index];

        // Write the color to the single on-board RGB LED
        driver
            .write(std::iter::once(color))
            .expect("Failed to write LED color");

        log::info!("LED: R={}, G={}, B={}", color.r, color.g, color.b);

        // Advance to the next colour in the cycle
        color_index = (color_index + 1) % colors.len();

        // Wait 500 ms before the next colour change
        FreeRtos::delay_ms(500);
    }
}
