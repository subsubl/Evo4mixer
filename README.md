# 🔥 Evo4mixer

[![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app/)
[![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)](https://reactjs.org/)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Vite](https://img.shields.io/badge/Vite-646CFF?style=for-the-badge&logo=vite&logoColor=white)](https://vitejs.dev/)

**Evo4mixer** is a high-performance, low-latency Linux mixer specifically engineered for the **Audient EVO 4** audio interface. Designed with a focus on stability and aesthetics, it bridges the gap between raw hardware control and a premium user experience.

> "A precise tool for the glitch, the light, and the sound."

---

## ✨ Features

- 🎚️ **ALSA-Driven Control**: Direct hardware manipulation without audio playback conflicts.
- 🎨 **RME-Inspired UI**: A professional, high-density interface designed for quick adjustments.
- ⚡ **Zero Latency**: Built on Tauri and Rust for near-instant response times.
- 📊 **Real-time Monitoring**: Visual feedback for input and output levels (WIP).
- 🐧 **Linux Native**: Deep integration with udev for seamless device permissions.

## 🛠️ Tech Stack

- **Frontend**: React (TSX) + Framer Motion for smooth animations.
- **Alternative Frontend**: Yew (Rust/Wasm) for pure Rust enthusiasts.
- **Backend**: Rust (Tauri) for secure USB and ALSA communication.
- **Styling**: Tailwind CSS + DaisyUI for a premium dark-mode aesthetic.

---

## 🚀 Getting Started

### Prerequisites

Ensure you have the following installed:
- [Tauri v2 Setup](https://tauri.app/v2/guides/getting-started/setup/linux/)
- [Rust & Cargo](https://rustup.rs/)
- [Node.js & npm](https://nodejs.org/)
- [Trunk](https://trunkrs.dev/) (for the Rust frontend)

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/subsubl/Evo4mixer.git
   cd Evo4mixer
   ```

2. **Configure Hardware Permissions:**
   Copy the udev rules to allow non-root access to the EVO 4:
   ```bash
   sudo cp 99-audient-evo.rules /etc/udev/rules.d/
   sudo udevadm control --reload-rules && sudo udevadm trigger
   ```

3. **Install Dependencies:**
   ```bash
   npm install
   ```

4. **Run in Development Mode:**
   ```bash
   npm run tauri dev
   ```

---

## 🎨 Design Philosophy

Inspired by the precision of RME interfaces and the chaotic beauty of glitch art. This isn't just a mixer; it's a part of your creative workflow—whether you're routing audio for an installation, mixing a live set, or tweaking levels for a lo-fi VHS production.

---

## 🤝 Contributing

This project is part of the **subsubl** creative ecosystem. Contributions are welcome! Whether it's adding support for more EVO features or refining the UI, feel free to open a PR.
