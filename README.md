<div align="center">
  <img src="logo.svg" width="200" alt="Shrike Logo">
</div>

<hr>

<h1 align="center">Shrike-rs</h1>

<p align="center">
  <strong>shrike-core</strong><br>
  <a href="https://crates.io/crates/shrike-core"><img src="https://img.shields.io/crates/v/shrike-core.svg" alt="Crates.io"></a>
  <a href="https://docs.rs/shrike-core"><img src="https://docs.rs/shrike-core/badge.svg" alt="Docs.rs"></a>
</p>

<p align="center">
  <strong>shrike-esp</strong><br>
  <a href="https://crates.io/crates/shrike-esp"><img src="https://img.shields.io/crates/v/shrike-esp.svg" alt="Crates.io"></a>
  <a href="https://docs.rs/shrike-esp"><img src="https://docs.rs/shrike-esp/badge.svg" alt="Docs.rs"></a>
</p>

<p align="center">
  <strong>shrike-rp</strong><br>
  <a href="https://crates.io/crates/shrike-rp"><img src="https://img.shields.io/crates/v/shrike-rp.svg" alt="Crates.io"></a>
  <a href="https://docs.rs/shrike-rp"><img src="https://docs.rs/shrike-rp/badge.svg" alt="Docs.rs"></a>
</p>

---

### Overview

**Shrike-rs** is a cross-platform library suite for **flashing FPGAs** directly from microcontrollers using Rust. It provides a standardized way to handle the power-up sequencing, hardware enabling, and SPI-based data transfer required to program FPGA bitstreams.

Designed for the `no_std` ecosystem, Shrike supports both synchronous (`embedded-hal`) and asynchronous (`embassy`) workflows, allowing you to integrate FPGA programming into your embedded applications with ease.

### Key Features

- **Universal Interface**: A consistent API for FPGA flashing across different hardware targets.
- **Power & Enable Management**: Built-in handling for power-on sequencing and hardware reset/enable pins.
- **Async & Sync Support**: Fully compatible with `embedded-hal` for simple blocking operations and `embassy` for high-performance asynchronous flashing.
- **Chunked Transfer**: Efficiently transfers bitstreams in 4096-byte chunks to minimize memory overhead.

### Crate Structure

- **`shrike-core`**: The core logic containing the `Flash` and `AsyncFlash` traits, along with the universal SPI implementation.
- **`shrike-esp`**: Specialized implementation for Espressif SoCs (ESP32 series) using `esp-hal`.
- **`shrike-rp`**: Specialized implementation for Raspberry Pi Silicon (RP2040 and RP235x) using `rp-hal`.

---

Developed for reliable, portable, and efficient FPGA bitstream deployment in the Rust embedded ecosystem.
