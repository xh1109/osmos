pub struct Cell {
    pub max_x: f64,
    pub max_y: f64,
    pub position: nalgebra::Point2<f64>,
    pub acceleration: nalgebra::Vector2<f64>,
    pub velocity: nalgebra::Vector2<f64>,
    pub energy: usize,
    pub sensor: crate::sensor::Sensor,
}

impl Cell {
    pub fn random(rng: &mut rand::rngs::ThreadRng, max_x: f64, max_y: f64) -> Self {
        let mut cell = Self {
            max_x,
            max_y,
            position: nalgebra::Point2::new(0.0, 0.0),
            acceleration: nalgebra::Vector2::new(0.0, 0.0),
            velocity: nalgebra::Vector2::new(0.0, 0.0),
            energy: rand::Rng::gen_range(rng, 1..=2),
            sensor: crate::sensor::Sensor::new(0.5),
        };
        cell.random_position(rng);
        cell
    }

    pub fn random_position(&mut self, rng: &mut rand::rngs::ThreadRng) {
        let random_x = rand::Rng::gen_range(rng, 0.0..=self.max_x);
        let random_y = rand::Rng::gen_range(rng, 0.0..=self.max_y);
        self.position.x = random_x;
        self.position.y = random_y;
    }

    pub fn get_max_velocity_magnitude(&self) -> f64 {
        let max_point_magnitude = nalgebra::Vector2::new(self.max_x, self.max_y).magnitude();
        max_point_magnitude / 1000.0 + (1.0 / self.energy as f64) * (max_point_magnitude / 2000.0)
    }
}
