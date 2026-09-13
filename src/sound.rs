use agb::include_wav;
use agb::sound::mixer::SoundData;

pub static ON: SoundData = include_wav!("sfx/select_003.wav");
pub static OFF: SoundData = include_wav!("sfx/select_004.wav");
pub static TICK: SoundData = include_wav!("sfx/tick_001.wav");
