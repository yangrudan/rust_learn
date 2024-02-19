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

trait Vehicle {
    fn get_price(&self) -> u64;
}

trait Car: Vehicle {
    fn model(&self) -> String;
}

struct TeslaRoadster {
    model: String,
    release_data: u16,
}

impl TeslaRoadster {
    fn new(model: &str, release_data: u16) -> Self {
        Self {
            model: model.to_string(),
            release_data,
        }
    }
}

impl Car for TeslaRoadster {
    fn model(&self) -> String {
        "Tesla Roadster".to_string()
    }
}

impl Vehicle for TeslaRoadster {
    fn get_price(&self) -> u64 {
        200_00
    }
}

struct Game;
struct Enemy;
struct Hero;

trait Loadable {
    fn init(&self);
}

impl Loadable for Enemy {
    fn init(&self) {
        println!("Enemy is loaded");
    }
}

impl Loadable for Hero {
    fn init(&self) {
        println!("Hero is loaded");
    }
}

impl Game {
    fn load<T: Loadable>(&self, entity: T) {
        entity.init();
    }
}

use std::fmt::Debug;

trait Eatable {
    fn eat(&self);
}

#[derive(Debug)]
struct Food<T>(T);

#[derive(Debug)]
struct Apple;

impl<T> Eatable for Food<T>
where
    T: Debug,
{
    fn eat(&self) {
        println!("Eating {:?}", self);
    }
}

fn eat<T: Eatable>(val: T) {
    val.eat();
}

fn main() {
    let audio = Audio("xxx.mp3".to_string());
    let vidio = Audio("xxx.mkv".to_string());
    audio.play();
    vidio.play();

    let my_roadster = TeslaRoadster::new("Tesla 🕑", 2020);
    println!(
        "{} is priced ad $ {}",
        my_roadster.model,
        my_roadster.get_price()
    );

    let game = Game;
    game.load(Enemy);
    game.load(Hero);

    let apple = Food(Apple);
    eat(apple);
}
