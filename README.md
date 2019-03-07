# `nrf52_tests`

Tests with the nrf52_dk board (nRF52832 Microcontroller)

# Hardware

- Microcontroller: nRF52832

# Dependencies

- arm-none-eabi-objcopy: Required to transform the `ELF` file to `HEX`
- nrfjprog: Required to install the `HEX` in the DK board.

# Debugging

- openocb compiled with jlink support.
- gdb

```bash
  ./debugger.sh
```
```bash
  cargo run --example <example_name>
```

# Installing

```bash
  cargo build
```
```bash
arm-none-eabi-objcopy \
  -O ihex \
  target/thumbv7em-none-eabihf/debug/watch \
  target/thumbv7em-none-eabihf/debug/watch.hex
```
```bash
  nrfjprog --family nRF52 --eraseall && \
  nrfjprog --family nRF52 --program target/thumbv7em-none-eabihf/debug/watch.hex && \
  nrfjprog --family nRF52 -r
```
