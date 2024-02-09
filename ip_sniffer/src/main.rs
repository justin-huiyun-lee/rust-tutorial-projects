enum MoveDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let snake_length = 2;
    let player_direction = MoveDirection::UP;

    while true {
        for i in 0..WIDTH {
            print!("#");
        }
        for i in 0..HEIGHT {
            println!("#");
        }
    }
}
