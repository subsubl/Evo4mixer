use alsa::mixer::{Mixer, SelemId, SelemChannelId};

pub struct EvoDevice {
    mixer: Mixer,
}

const CARD_NAME: &str = "EVO4";
const MIXER_NAME: &str = "EVO4  Playback Volume"; // Note: double space in official name

impl EvoDevice {
    pub fn open() -> Result<Self, String> {
        // Open the ALSA mixer for the EVO 4 card
        let mixer = Mixer::new(CARD_NAME, false)
            .map_err(|e| format!("Failed to open ALSA mixer: {}", e))?;
        
        Ok(EvoDevice { mixer })
    }
    
    pub fn set_volume(&self, input: u8, output: u8, db: f32) -> Result<(), String> {
        // Volume mapping:
        // - Input range: -128 dB to 0 dB
        // - ALSA range: 0 to 254 (0 = -127dB, 254 = 0dB)
        
        let mut vol_db_clamped = db;
        if vol_db_clamped <= -100.0 {
            vol_db_clamped = -127.0;
        } else if vol_db_clamped > 0.0 {
            vol_db_clamped = 0.0;
        }
        
        // Convert dB to ALSA scale (0-254)
        let alsa_val = ((vol_db_clamped + 127.0) * (254.0 / 127.0)).round() as i64;
        let alsa_val = alsa_val.clamp(0, 254);
        
        // Find the mixer element
        let selem_id = SelemId::new(MIXER_NAME, 0);
        let selem = self.mixer.find_selem(&selem_id)
            .ok_or_else(|| format!("Mixer element '{}' not found", MIXER_NAME))?;
        
        // EVO 4 mixer matrix: The ALSA control has 4 values
        // We need to determine which matrix position this input/output combination represents
        // For now, set all channels to the same value for testing
        // TODO: Properly map matrix positions to ALSA channels
        
        // The ALSA API doesn't expose direct indexing, so we'll use the channel enum
        // Map our indices to ALSA channels (this is a simplified approach)
        let channel = match (output * 8 + input) % 4 {
            0 => SelemChannelId::FrontLeft,
            1 => SelemChannelId::FrontRight,
            2 => SelemChannelId::FrontCenter,
            3 => SelemChannelId::RearLeft,
            _ => SelemChannelId::FrontLeft,
        };
        
        selem.set_playback_volume(channel, alsa_val)
            .map_err(|e| format!("Failed to set volume: {}", e))?;
        
        Ok(())
    }
}
