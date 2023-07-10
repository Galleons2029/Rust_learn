// mod demo;

fn main() {
    let shadow_num = 5;

    let shadow_num = shadow_num + 5;

    let shadow_num = shadow_num * 2;

    let is = 1 > 4;
    println!("The number is {}.", shadow_num);
    println!("The number is {}.",  is);

    let we_load = WebEvent::WELoad(true);

    let click = MouseClick { x: 100, y: 250 };

    let we_click = WebEvent::WEClick(click);
}

struct KeyPress(String, char);

struct MouseClick {
    x: i64,
    y: i64
}

enum WebEvent {
    WELoad(bool),
    WEClick(MouseClick),
    WEKeys(KeyPress)
}



use serde::Deserialize;

#[derive(Deserialize)]
struct Gcdparameters {
    n: u64,
    m: u64,
}

