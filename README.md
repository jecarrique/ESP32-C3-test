# ESP32-C3 RGB LED Blink — Rust

Proyecto Rust para la placa de desarrollo **ESP32-C3** que hace blink en el LED RGB integrado (WS2812B) usando ESP-IDF.

## Hardware

| Componente | Detalle |
|---|---|
| Placa | ESP32-C3 DevKit (ESP32-C3-DevKitC-02, ESP32-C3-DevKitM-1, SuperMini, etc.) |
| LED RGB | WS2812B integrado en **GPIO8** |
| Protocolo | RMT (canal 0) |

## Comportamiento

El LED RGB cicla por los siguientes colores, con 500 ms cada uno:

1. 🔴 Rojo `(255, 0, 0)`
2. 🟢 Verde `(0, 255, 0)`
3. 🔵 Azul `(0, 0, 255)`
4. ⚫ Apagado `(0, 0, 0)` ← efecto blink

## Prerrequisitos

### 1. Rust nightly con soporte RISC-V

```bash
# Instalar rustup si no está instalado
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Instalar la toolchain nightly con rust-src (necesario para build-std)
rustup toolchain install nightly --component rust-src
```

### 2. Herramientas ESP

```bash
# Linker proxy para ESP-IDF
cargo install ldproxy

# Herramienta de flash y monitor serial
cargo install espflash
```

### 3. ESP-IDF

El crate `embuild` descarga y configura automáticamente ESP-IDF v5.2.1 durante
la primera compilación. No se necesita instalar ESP-IDF manualmente.

> **Nota:** la primera compilación puede tardar varios minutos porque descarga
> y compila el SDK completo de ESP-IDF.

## Compilar y flashear

```bash
# Solo compilar
cargo build --release

# Compilar y flashear (el runner es espflash, definido en .cargo/config.toml)
cargo run --release
```

O manualmente con `espflash`:

```bash
espflash flash --monitor \
    target/riscv32imc-esp-espidf/release/esp32c3-rgb-blink
```

## Estructura del proyecto

```
├── src/
│   └── main.rs              # Lógica principal: blink del LED RGB
├── .cargo/
│   └── config.toml          # Target RISC-V, linker, runner, ESP-IDF version
├── Cargo.toml               # Dependencias del proyecto
├── build.rs                 # Script de build para integrar con ESP-IDF
├── rust-toolchain.toml      # Fija la toolchain a nightly
├── sdkconfig.defaults        # Configuración del SDK de ESP-IDF
└── .gitignore               # Excluye artefactos de build
```

## Dependencias principales

| Crate | Uso |
|---|---|
| [`esp-idf-svc`](https://crates.io/crates/esp-idf-svc) | HAL y servicios sobre ESP-IDF (incluye FreeRTOS delay) |
| [`ws2812-esp32-rmt-driver`](https://crates.io/crates/ws2812-esp32-rmt-driver) | Driver WS2812B usando el periférico RMT del ESP32 |
| [`smart-leds`](https://crates.io/crates/smart-leds) | Trait `SmartLedsWrite` y tipo `RGB8` |
| [`embuild`](https://crates.io/crates/embuild) | Descarga y configura ESP-IDF automáticamente |
