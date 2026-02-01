# 🔥 Evo4mixer

[![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app/)
[![Yew](https://img.shields.io/badge/Yew-000000?style=for-the-badge&logo=rust&logoColor=white)](https://yew.rs/)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)

**Evo4mixer** is a high-performance, low-latency Linux mixer specifically engineered for the **Audient EVO 4** audio interface. 

---

## ⚡ Recent Progress: The "Deep Dive" Protocol Update

We have successfully reverse-engineered the core control protocol of the EVO 4 by analyzing both the Windows driver (Thesycon) and community-driven USB captures. 

### Key Discoveries:
- **Unit 58 (0x3A)**: Target for Input-side controls (Mic Gain, Phantom Power).
- **Unit 59 (0x3B)**: Target for Output-side controls (Headphone/Master Volume).
- **Control Requests**: Uses UAC2 Class-Specific `SET_CUR` requests (0x21, 0x01) with 4-byte payloads.
- **Phantom Power (48V)**: Controlled via `wValue` 0x00XX (where XX is channel) on Unit 58.
- **Mic Gain**: Controlled via `wValue` 0x01XX on Unit 58, range 0x00 to 0x31.

### Features Implemented:
- 🦀 **Pure Rust Stack**: Yew frontend + Tauri v2 backend.
- 🛠️ **Direct USB Backend**: `rusb` implementation for raw control transfers, bypassing ALSA limitations.
- ⚡ **Command API**: Exposes `set_gain`, `toggle_phantom`, and `set_master_volume` to the frontend.

---

## 🚀 Getting Started

### Prerequisites

- [Tauri v2 Setup](https://tauri.app/v2/guides/getting-started/setup/linux/)
- [Trunk](https://trunkrs.dev/) (`cargo install --locked trunk`)
- `libusb-1.0-0-dev` (Linux).

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
