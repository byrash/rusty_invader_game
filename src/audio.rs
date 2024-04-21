use rusty_audio::Audio;

pub enum AudioActions {
    StartUp,
    Lose,
    Explode,
    Move,
    Pew,
    Win,
}

pub fn play(audio_actions: AudioActions, audio: &mut Audio) {
    match audio_actions {
        AudioActions::StartUp => audio.play("startup"),
        AudioActions::Lose => audio.play("lose"),
        AudioActions::Explode => audio.play("explode"),
        AudioActions::Move => audio.play("move"),
        AudioActions::Pew => audio.play("pew"),
        AudioActions::Win => audio.play("win"),
    }
}

pub fn create_audio() -> Audio {
    let mut audio = Audio::new();
    audio.add("explode", "audio/explode.wav");
    audio.add("lose", "audio/lose.wav");
    audio.add("move", "audio/move.wav");
    audio.add("pew", "audio/pew.wav");
    audio.add("startup", "audio/startup.wav");
    audio.add("win", "audio/win.wav");
    audio
}
