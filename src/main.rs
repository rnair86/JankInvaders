use std::error::Error;
use rusty_audio::Audio;

fn main() -> Result<(),Box<dyn Error>> {
    //println!("Hello, world!");
    let mut audio = Audio::new();
    audio.add("explode","explode.wav");
    audio.add("lose","lose.wav");
    audio.add("move","move.wav");
    audio.add("pew","pew.wav");
    audio.add("startup","startup.wav");
    audio.add("win","win.wav");

    audio.play("startup");

    //cleanup
    audio.wait();
    Ok(())

}
