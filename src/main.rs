use soloud::*;
use std::thread::spawn;
#[macro_use] extern crate rocket;

#[get("/")]
fn play() {
    spawn(|| {
        let sl = Soloud::default().unwrap();
        let mut wav = audio::Wav::default();
        wav.load_mem(include_bytes!("../doorbell.wav").to_vec()).unwrap();
        sl.play(&wav);
        while sl.voice_count() > 0 {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }); 
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/doorbell", routes![play])
}