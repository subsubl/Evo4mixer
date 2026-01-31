use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub struct AudioMonitor {
    pub levels: Arc<Mutex<Vec<f32>>>,
}

impl AudioMonitor {
    pub fn new() -> Self {
        AudioMonitor {
            levels: Arc::new(Mutex::new(vec![0.0; 4])),
        }
    }

    pub fn start(&self) {
        let levels = Arc::clone(&self.levels);
        std::thread::spawn(move || {
            let host = cpal::default_host();
            let device = host.default_input_device().expect("no input device available");
            let config = device.default_input_config().expect("failed to get default input config");

            let stream = device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &_| {
                    let mut l = levels.lock().unwrap();
                    // Just a dummy VU calculation
                    for (i, val) in data.iter().enumerate() {
                        if i < 4 {
                            l[i] = val.abs();
                        }
                    }
                },
                |err| eprintln!("stream error: {}", err),
                None
            ).unwrap();

            stream.play().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(u64::MAX));
        });
    }
}
