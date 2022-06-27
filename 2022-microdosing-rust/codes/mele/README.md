# mele

A simple music-box used during the demo.

## Setup

Assuming you've got Arduino Uno:

- Connect a piezo speaker to `D11` + `GND`,
- Compile & flash,
- Using a male-to-male cable, connect `5V` to `A0`, `A1`, `A2` or `A3` (to choose the song),
- Restart the device (e.g. using the `RESET` pin on the board),
- Listen 🎶.

## Compiling & Flashing

Assuming you've got [ravedude](https://crates.io/crates/ravedude) installed:

```bash
$ cargo run
```
