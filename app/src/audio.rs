use general_audio_static::{AudioPlayer, StaticAudioPlayer, StaticHandle, StaticSound};
use maplit::hashmap;
use std::collections::HashMap;

pub type AppAudioPlayer = Option<StaticAudioPlayer>;
pub type AppSound = Option<StaticSound>;
pub type AppHandle = Option<StaticHandle>;

const EXPLOSION: &[u8] = include_bytes!("./audio/explosion.ogg");
const FIBERITRON: &[u8] = include_bytes!("./audio/fiberitron-loop.ogg");

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Audio {
    Explosion,
    Fiberitron,
}

pub struct AudioTable {
    map: HashMap<Audio, AppSound>,
}

impl AudioTable {
    pub fn new(audio_player: &AppAudioPlayer) -> Self {
        let map = hashmap![
            Audio::Explosion => audio_player.load_sound(EXPLOSION),
            Audio::Fiberitron => audio_player.load_sound(FIBERITRON),
        ];
        Self { map }
    }
    pub fn get(&self, audio: Audio) -> &AppSound {
        self.map.get(&audio).unwrap()
    }
}
