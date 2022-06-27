# distance

A distance meter based on HC-SR04 that sends measurements to UART.

## Setup

Assuming you've got Arduino Uno:

- Connect HC-SR04's `trig` pin to `D9`,
- Connect HC-SR04's `echo` pin to `D8`,
- Compile & flash,
- Watch measurements on UART.

## Compiling & Flashing

Assuming you've got [ravedude](https://crates.io/crates/ravedude) installed:

```bash
$ cargo run
```

## Usage

