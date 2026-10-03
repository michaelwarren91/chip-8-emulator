use sdl2::{
    Sdl,
    audio::{AudioCallback, AudioDevice, AudioSpecDesired, AudioStatus},
};

use super::AudioBackend;

struct SquareWave {
    phase: f32,
    phase_increment: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for sample in out.iter_mut() {
            *sample = if self.phase < 0.5 { 0.25 } else { -0.25 };

            self.phase = (self.phase + self.phase_increment) % 1.0;
        }
    }
}

pub struct SdlAudio {
    audio_device: AudioDevice<SquareWave>,
}

impl SdlAudio {
    pub fn new(sdl_context: &Sdl) -> Self {
        let audio_subsystem = sdl_context.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: Some(1024),
        };

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| SquareWave {
                phase: 0.0,
                phase_increment: 440.0 / spec.freq as f32,
            })
            .expect("Failed to create audio device");

        Self {
            audio_device: device,
        }
    }
}

impl AudioBackend for SdlAudio {
    fn play_sound(&mut self) {
        self.audio_device.resume();
    }

    fn stop_sound(&mut self) {
        self.audio_device.pause();
    }

    fn is_playing(&self) -> bool {
        self.audio_device.status() == AudioStatus::Playing
    }
}
