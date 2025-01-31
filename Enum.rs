enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let my_direction = Direction::Left; // Explicitly use Direction::

    match my_direction {
        Direction::Up => println!("Going up"),
        Direction::Left => println!("Going Left"),
        Direction::Right => println!("Going Right"),
        Direction::Down => println!("Going Down"),
    }
}

