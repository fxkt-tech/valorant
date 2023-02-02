mod game;
mod system;

use game::weapon::{sheriff::Sheriff, Weapon};

fn main() {
    let mut sub_gun = Sheriff::new();
    for _ in 0..24 {
        sub_gun.fire();
    }
    sub_gun.print_magazine();
}
