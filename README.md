# 🔥 Evo4mixer

[![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app/)
[![Yew](https://img.shields.io/badge/Yew-000000?style=for-the-badge&logo=rust&logoColor=white)](https://yew.rs/)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)

**Evo4mixer** is a high-performance, low-latency Linux mixer specifically engineered for the **Audient EVO 4** audio interface. 

This project aims to provide a full-featured alternative to the Audient Windows/macOS dashboard, moving beyond basic ALSA controls to direct USB/HID manipulation for features like Smartgain, Phantom Power, and the internal Matrix Mixer.

---

## ⚡ Recent Progress: The "Ghidra" Update

- 🦀 **Pure Rust Stack**: Migrated the entire project to **Yew** (frontend) and **Tauri** (backend). No more JavaScript/TypeScript overhead.
- 🛠️ **Direct USB Control**: Transitioned from ALSA-only volume control to a raw `rusb` implementation targeting specific UAC2 Units (Feature Units 10/11, Mixer Unit 60, and Extension Units 50-59).
- 🔍 **Reverse Engineering**: Currently leveraging Ghidra analysis of the Windows driver to map proprietary Audient control requests for:
  - **Phantom Power (48V)** toggle.
  - **Smartgain** trigger and status.
  - **Input Link** (Stereo/Mono).
  - **Internal Mixer Matrix** routing.

---

## ✨ Features

- 🎚️ **Direct USB Manipulation**: Bypassing ALSA for proprietary hardware features.
- 🎨 **RME-Inspired UI**: A professional, high-density interface built with pure Rust (Yew).
- ⚡ **Zero Latency**: Sub-millisecond control response times.
- 🐧 **Linux Native**: Deep integration with udev for device permissions.

## 🛠️ Tech Stack

- **Frontend**: [Yew](https://yew.rs/) (Rust/Wasm).
- **Backend**: [Tauri v2](https://tauri.app/) + [rusb](https://docs.rs/rusb/) for low-level USB.

---

## 🚀 Getting Started

### Prerequisites

- [Tauri v2 Setup](https://tauri.app/v2/guides/getting-started/setup/linux/)
- [Trunk](https://trunkrs.dev/) (`cargo install --locked trunk`)
- [libusb](https://libusb.info/) development headers.

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/subsubl/Evo4mixer.git
   cd Evo4mixer
   ```

2. **Configure Hardware Permissions:**
   ```bash
   sudo cp 99-audient-evo.rules /etc/udev/rules.d/
   sudo udevadm control --reload-rules && sudo udevadm trigger
   ```

3. **Run:**
   ```bash
   npm run tauri dev
   ```

---

*Updated with fire by Prometheus 🔥*
