use crate::traffic::TrafficState;
use rand::Rng;

#[derive(Debug)]
pub enum ComingFrom {
    North,
    East,
    South,
    West,
    Random,
}
#[derive(Debug)]
pub enum Going {
    Straight,
    Right,
    Left,
}

#[derive(Debug)]
pub struct Car {
    pub coming_from: ComingFrom,
    pub going: Going,
    pub x: f64,
    pub y: f64,
    pub moving: bool,
}

impl Car {
    pub fn new(coming_from: ComingFrom) -> Car {
        let mut rng = rand::thread_rng();
        let going = match rng.gen_range(0..3) {
            0 => Going::Straight,
            1 => Going::Right,
            _ => Going::Left,
        };
        return match coming_from {
            ComingFrom::North => Car {
                coming_from,
                going,
                x: 450.0,
                y: 50.0,
                moving: true,
            },
            ComingFrom::East => Car {
                coming_from,
                going,
                x: 950.0,
                y: 550.0,
                moving: true,
            },
            ComingFrom::South => Car {
                coming_from,
                going,
                x: 600.0,
                y: 50.0,
                moving: true,
            },
            _ => Car {
                coming_from,
                going,
                x: 50.0,
                y: 500.0,
                moving: true,
            },
        };
    }

    pub fn random(ts: &mut TrafficState) -> Car {
        let mut rng = rand::thread_rng();
        loop {
            match rng.gen_range(0..4) {
                0 => {
                    if ts.from_north.len() < 8 && ts.allow_new_car[0] {
                        ts.allow_new_car[0] = false;
                        return Car::new(ComingFrom::North);
                    }
                }
                1 => {
                    if ts.from_east.len() < 8 && ts.allow_new_car[1] {
                        ts.allow_new_car[1] = false;
                        return Car::new(ComingFrom::East);
                    }
                }
                2 => {
                    if ts.from_south.len() < 8 && ts.allow_new_car[2] {
                        ts.allow_new_car[2] = false;
                        return Car::new(ComingFrom::South);
                    }
                }
                _ => {
                    if ts.from_west.len() < 8 && ts.allow_new_car[3] {
                        ts.allow_new_car[3] = false;
                        return Car::new(ComingFrom::West);
                    }
                }
            }
        }
    }

    pub fn move_car(&mut self) {
        match self.coming_from {
            ComingFrom::North => {
                if self.y >= 500.0 {
                    match self.going {
                        Going::Straight => self.down(),
                        Going::Right => self.left(),
                        Going::Left => self.right(),
                    }
                } else {
                    self.down();
                }
            }
            ComingFrom::East => {
                if self.x <= 500.0 {
                    match self.going {
                        Going::Straight => self.left(),
                        Going::Right => self.up(),
                        Going::Left => self.down(),
                    }
                } else {
                    self.left()
                }
            }
            ComingFrom::South => {
                if self.y <= 500.0 {
                    match self.going {
                        Going::Straight => self.up(),
                        Going::Right => self.right(),
                        Going::Left => self.left(),
                    }
                } else {
                    self.up();
                }
            }
            _ => {
                if self.x >= 500.0 {
                    match self.going {
                        Going::Straight => self.right(),
                        Going::Right => self.down(),
                        Going::Left => self.up(),
                    }
                } else {
                    self.right()
                }
            }
        }
    }

    fn up(&mut self) {
        self.y += 1.0
    }
    fn down(&mut self) {
        self.y -= 1.0
    }
    fn right(&mut self) {
        self.x += 1.0
    }
    fn left(&mut self) {
        self.x -= 1.0
    }
}
