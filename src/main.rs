#![no_std]
#![no_main]

// Manejador de pánicos para volcar errores por consola en caso de crash
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    peripherals::Peripherals,
    prelude::*,
    rmt::Rmt,
};
use esp_hal_smartleds::SmartLedsAdapter;
use smart_leds::{SmartLedsWrite, RGB8};

#[entry]
fn main() -> ! {
    // Tomamos el control de los periféricos del hardware
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    
    // Configuramos los relojes del sistema (esencial para calcular los tiempos del RMT)
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Inicializamos la utilidad de delay basada en los ciclos de reloj
    let delay = Delay::new(&clocks);

    esp_println::println!("ESP32-C3 RGB LED blink starting (no_std)!");

    // Inicializamos el periférico RMT (Remote Control) a 80 MHz para los timmings del WS2812B
    let rmt = Rmt::new(peripherals.RMT, 80.MHz(), &clocks).unwrap();
    
    // Conectamos el canal 0 del RMT al pin GPIO8 (el LED integrado en la placa)
    // El adaptador de esp-hal-smartleds se encarga de traducir los bytes a pulsos RMT
    let mut led = SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO8, &clocks);

    // Paleta de colores para el ciclo (R, G, B)
    let colors: [RGB8; 4] = [
        RGB8::new(255, 0, 0), // Rojo
        RGB8::new(0, 255, 0), // Verde
        RGB8::new(0, 0, 255), // Azul
        RGB8::new(0, 0, 0),   // Apagado
    ];

    let mut color_index = 0usize;

    loop {
        let color = colors[color_index];

        // Escribimos el color actual en el LED usando un iterador de un solo elemento
        led.write(core::iter::once(color)).unwrap();

        // En no_std usamos esp_println en lugar del macro de log tradicional
        esp_println::println!("LED: R={}, G={}, B={}", color.r, color.g, color.b);

        // Avanzamos al siguiente color
        color_index = (color_index + 1) % colors.len();

        // Pausa de 500 milisegundos
        delay.delay_ms(500u32);
    }
}
