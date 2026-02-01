use rusb::{DeviceHandle, GlobalContext};

pub struct EvoDevice {
    handle: DeviceHandle<GlobalContext>,
}

const VENDOR_ID: u16 = 0x2708;
const PRODUCT_ID: u16 = 0x0006;

// Verified Unit IDs from lsusb and community tools
const UNIT_ID_INPUT: u8 = 0x3A;  // 58
const UNIT_ID_OUTPUT: u8 = 0x3B; // 59
const UNIT_ID_MIXER: u8 = 0x32;  // 50

impl EvoDevice {
    pub fn open() -> Result<Self, String> {
        let handle = rusb::open_device_with_vid_pid(VENDOR_ID, PRODUCT_ID)
            .ok_or_else(|| "EVO 4 device not found. Check connections and udev rules.".to_string())?;
        
        // Claim the control interface (usually 0)
        // Note: In a production app, we might need to detach the kernel driver if it's active.
        // For now, assume the udev rule and ALSA co-existence work.
        handle.claim_interface(0)
            .map_err(|e| format!("Failed to claim USB interface: {}", e))?;
            
        Ok(EvoDevice { handle })
    }

    /// Set Input Gain for a channel (1-indexed)
    /// Range: 0x00 to 0x31 (likely mapping to 0-50dB)
    pub fn set_input_gain(&self, channel: u8, gain: u8) -> Result<(), String> {
        // CS=0x01 (Gain), Channel=channel-1
        let w_value = (0x01u16 << 8) | ((channel - 1) as u16);
        let w_index = ((UNIT_ID_INPUT as u16) << 8) | 0x00;
        let data = [0x00, gain, 0x00, 0x00]; // 4-byte payload pattern from reverse engineering

        self.send_control_request(w_value, w_index, &data)
    }

    /// Toggle Phantom Power (48V) for a channel
    pub fn set_phantom_power(&self, channel: u8, enabled: bool) -> Result<(), String> {
        // CS=0x00 (Phantom), Channel=channel-1
        let w_value = (0x00u16 << 8) | ((channel - 1) as u16);
        let w_index = ((UNIT_ID_INPUT as u16) << 8) | 0x00;
        let val = if enabled { 0x01 } else { 0x00 };
        let data = [val, 0x00, 0x00, 0x00];

        self.send_control_request(w_value, w_index, &data)
    }

    /// Set Master/Headphone Volume
    /// Range: 0x00 to 0xFF
    pub fn set_output_volume(&self, volume: u8) -> Result<(), String> {
        let w_value = 0x0000;
        let w_index = ((UNIT_ID_OUTPUT as u16) << 8) | 0x00;
        // Pattern: [0x00, vol, 0xff, 0xff]
        let data = [0x00, volume, 0xff, 0xff];

        self.send_control_request(w_value, w_index, &data)
    }

    /// Set Mixer Routing/Matrix
    /// Unit 60 is a 6x2 Mixer Unit (Mix 1)
    pub fn set_mixer_node(&self, node_index: u8, volume_db: f32) -> Result<(), String> {
        // UAC2 Mixer Control (CS=0x01)
        // wValue: (CS << 8) | 0x00
        // wIndex: (UnitID << 8) | Interface
        let w_value = (0x01u16 << 8);
        let w_index = (60u16 << 8) | 0x00;
        
        // UAC2 Volume: 16-bit signed, 1/256 dB units
        let vol_raw = (volume_db * 256.0) as i16;
        let data = vol_raw.to_le_bytes();

        // For Mixer Units, the request often targets specific nodes or the whole matrix.
        // Audient likely uses a proprietary mapping here since it's a 6x2 matrix.
        self.send_control_request(w_value, w_index, &data)
    }

    fn send_control_request(&self, w_value: u16, w_index: u16, data: &[u8]) -> Result<(), String> {
        let timeout = std::time::Duration::from_millis(1000);
        
        // bmRequestType: 0x21 (Host-to-Interface, Class-specific)
        // bRequest: 0x01 (SET_CUR)
        self.handle.write_control(0x21, 0x01, w_value, w_index, data, timeout)
            .map_err(|e| format!("USB request failed (val: 0x{:04x}, idx: 0x{:04x}): {}", w_value, w_index, e))?;
        
        Ok(())
    }
}
