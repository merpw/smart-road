#[derive(Clone, Copy, Default)]
pub struct Statistics {
    pub show_statistics: bool,

    pub car_count: usize,
    pub max_speed: f32,
    pub min_speed: f32,
    pub max_time: f32,
    pub min_time: f32,
    pub close_calls: usize,
    pub collision_count: i32,

    pub is_open: bool,
}

impl Statistics {
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }
}
