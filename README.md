# Evo4mixer

Advanced Linux mixer for Audient EVO 4 audio interfaces.

## Overview
Evo4mixer is a low-latency Linux control application for the Audient EVO 4. It provides a native interface to manage hardware parameters including gain, phantom power, and internal routing.

## Technology Stack
- **Frontend**: Yew (Rust/WebAssembly)
- **Backend**: Tauri v2
- **Hardware Interface**: Direct USB control via `rusb`

## Installation
1. Install dependencies: `libusb-1.0-0-dev`, `trunk`.
2. Configure udev rules using the provided `99-audient-evo.rules`.
3. Build and run:
   ```bash
   npm install
   npm run tauri dev
   ```

## Project Status
This project is currently a **Work In Progress (WIP)**. Core USB protocol mapping is partially implemented.
