pub mod sdl_audio;

pub trait AudioBackend {
    fn play_sound(&mut self);
    fn stop_sound(&mut self);
    fn is_playing(&self) -> bool;
}
