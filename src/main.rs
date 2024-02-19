///将代码中类型的共享行为和公共属性与其自身隔离开
struct Audio(String);
struct Video(String);

trait Playable {
    fn play(&self);
    fn pause() {
        println!("Paused");
    }
}

impl Playable for Audio {
    fn play(&self) {
        println!("Now is Audio {} playing!", self.0);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("Now is Video {} playing!", self.0);
    }
}

fn main() {
    let audio = Audio("xxx.mp3".to_string());
    let vidio = Audio("xxx.mkv".to_string());
    audio.play();
    vidio.play();
}
