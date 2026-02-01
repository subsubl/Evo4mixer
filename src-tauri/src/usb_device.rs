use rusb::{DeviceHandle, GlobalContext};

pub struct EvoDevice {
    handle: DeviceHandle<GlobalContext>,
}

const VENDOR_ID: u16 = 0x2708;
const PRODUCT_ID: u16 = 0x0006;

// USB Unit IDs from lsusb
const UNIT_ID_MIXER: u8 = 60;
const UNIT_ID_INPUT_1: u8 = 11;
const UNIT_ID_INPUT_2: u8 = 10;
const UNIT_ID_EXTENSION_50: u8 = 50;

impl EvoDevice {
    pub fn open() -> Result<Self, String> {
        let handle = rusb::open_device_with_vid_pid(VENDOR_ID, PRODUCT_ID)
            .ok_or_else(|| "EVO 4 device not found or permission denied".to_string())?;
        
        Ok(EvoDevice { handle })
    }

    /// Set volume for a specific unit (Feature Unit or Mixer Unit)
    /// This is a generic UAC2 Volume Control request
    pub fn set_unit_volume(&self, unit_id: u8, channel: u8, volume_db: f32) -> Result<(), String> {
        // UAC2 Volume is 1/256 dB units, 16-bit signed
        let vol_raw = (volume_db * 256.0) as i16;
        let data = vol_raw.to_le_bytes();

        // bRequest: 0x01 (CUR), 0x02 (RANGE), etc.
        // bmRequestType: 0x21 (Host-to-Interface, Class-specific)
        // wValue: Control Selector (CS) in high byte, Channel Number in low byte
        // Control Selector for Volume is 0x02 in UAC2
        let w_value = (0x02u16 << 8) | (channel as u16);
        let w_index = ((unit_id as u16) << 8) | 0x00; // Interface 0 (Audio Control)

        let timeout = std::time::Duration::from_millis(1000);
        
        self.handle.write_control(0x21, 0x01, w_value, w_index, &data, timeout)
            .map_err(|e| format!("USB Control Transfer failed: {}", e))?;
        
        Ok(())
    }

    /// Toggle Phantom Power (48V)
    /// Likely via an Extension Unit or Feature Unit
    pub fn set_phantom_power(&self, channel: u8, enabled: bool) -> Result<(), String> {
        // This is a placeholder for the specific Request ID found in the Windows driver
        // Often these are handled via Extension Units (e.g., Unit 58)
        let unit_id = 58; 
        let data = [if enabled { 1 } else { 0 }];
        
        // CS might be different for phantom power
        let w_value = (0x01u16 << 8) | (channel as u16);
        let w_index = ((unit_id as u16) << 8) | 0x00;

        let timeout = std::time::Duration::from_millis(1000);
        
        self.handle.write_control(0x21, 0x01, w_value, w_index, &data, timeout)
            .map_err(|e| format!("Failed to toggle Phantom Power: {}", e))?;
            
        Ok(())
    }
}
