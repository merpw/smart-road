use crate::config::{WINDOW_HEIGHT, WINDOW_WIDTH};
use rand::Rng;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ComingFrom {
    North,
    East,
    South,
    West,
}

const COMING_FROM: [ComingFrom; 4] = [
    ComingFrom::North,
    ComingFrom::East,
    ComingFrom::South,
    ComingFrom::West,
];

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Going {
    Straight,
    Right,
    Left,
}

const GOING: [Going; 3] = [Going::Straight, Going::Right, Going::Left];

#[derive(Debug)]
pub struct Car {
    pub coming_from: ComingFrom,
    pub going: Going,
    pub x: f32,
    pub y: f32,

    pub moving: bool,
}

impl Car {
    pub fn new(coming_from: ComingFrom) -> Car {
        let going = GOING[rand::thread_rng().gen_range(0..GOING.len())];

        match coming_from {
            ComingFrom::North => Car {
                coming_from,
                going,
                moving: true,

                x: WINDOW_WIDTH as f32 / 2.0,
                y: 0.0,
            },
            ComingFrom::East => Car {
                coming_from,
                going,
                moving: true,

                x: WINDOW_WIDTH as f32,
                y: WINDOW_HEIGHT as f32 / 2.0,
            },
            ComingFrom::South => Car {
                coming_from,
                going,
                moving: true,

                x: WINDOW_WIDTH as f32 / 2.0,
                y: WINDOW_HEIGHT as f32,
            },
            ComingFrom::West => Car {
                coming_from,
                going,
                moving: true,

                x: 0.0,
                y: WINDOW_HEIGHT as f32 / 2.0,
            },
        }
    }

    // pub fn random(ts: &mut TrafficState) -> Car {
    //     let mut rng = rand::thread_rng();
    //     loop {
    //         match rng.gen_range(0..4) {
    //             0 => {
    //                 if ts.from_north.len() < 8 && ts.allow_new_car[0] {
    //                     ts.allow_new_car[0] = false;
    //                     return Car::new(ComingFrom::North);
    //                 }
    //             }
    //             1 => {
    //                 if ts.from_east.len() < 8 && ts.allow_new_car[1] {
    //                     ts.allow_new_car[1] = false;
    //                     return Car::new(ComingFrom::East);
    //                 }
    //             }
    //             2 => {
    //                 if ts.from_south.len() < 8 && ts.allow_new_car[2] {
    //                     ts.allow_new_car[2] = false;
    //                     return Car::new(ComingFrom::South);
    //                 }
    //             }
    //             _ => {
    //                 if ts.from_west.len() < 8 && ts.allow_new_car[3] {
    //                     ts.allow_new_car[3] = false;
    //                     return Car::new(ComingFrom::West);
    //                 }
    //             }
    //         }
    //     }
    // }

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
