use rand::Rng;

const COHERENCE_FACTOR: f32 = 0.01;
const SEPARATION_FACTOR: f32 = 0.01;
const ALIGNMENT_FACTOR: f32 = 0.01;
const OBSTACLE_AVOIDANCE_FACTOR: f32 = 5.0;
const VISUAL_RADIUS: f32 = 50.0;
const MAX_SPEED: f32 = 0.5;

#[derive(Debug)]
#[derive(Clone)]
pub struct Boid {
    pub pos: Vector2,
    pub vel: Vector2,
}

impl Boid {
    fn new(pos: Vector2, vel: Vector2) -> Self {
        Self {
            pos,
            vel
        }
    }

    fn update(&mut self, boids: &Vec<Boid>, obstacles: &Vec<Obstacle>, width: &f32, height: &f32) {
        let mut alignment_vector = self.get_alignment_vector(boids);
        let mut separation_vector = self.get_separation_vector(boids);
        let mut coherence_vector = self.get_cohesion_vector(boids);
        let mut obstacle_avoidance_vector = self.get_obstacle_avoidance_vector(&obstacles);

        alignment_vector.mult(&ALIGNMENT_FACTOR);
        //separation_vector.mult(&SEPARATION_FACTOR); //Last Step
        coherence_vector.mult(&COHERENCE_FACTOR);
        obstacle_avoidance_vector.mult(&OBSTACLE_AVOIDANCE_FACTOR);

        let mut acceleration = Vector2::new(0.0, 0.0);
        acceleration.add(&alignment_vector);
        //acceleration.add(&separation_vector);
        acceleration.add(&coherence_vector);
        acceleration.add(&obstacle_avoidance_vector);

        self.vel = self.vel.add(&acceleration);
        self.vel.limit(&MAX_SPEED);

        self.pos.add(&self.vel);

        if self.pos.x < -width {
            self.pos.x = *width;
        }
        else if self.pos.x > *width {
            self.pos.x = -width;
        }

        if self.pos.y < -height {
            self.pos.y = *height;
        }
        else if self.pos.y > *height {
            self.pos.y = -height;
        }

    }

    fn get_alignment_vector(&self, flock: &Vec<Boid>) -> Vector2 {
        let mut alignment_vector = Vector2::new(0.0, 0.0);
        for boid in flock.iter().filter(|b| b.pos.distance_to(&self.pos) < VISUAL_RADIUS && b.pos.distance_to(&self.pos) > 0.0) {
            alignment_vector.add(&boid.vel);
        }
        alignment_vector.limit(&MAX_SPEED);
        alignment_vector
    }

    fn get_cohesion_vector(&self, flock: &Vec<Boid>) -> Vector2 {
        let mut cohesion_vector = Vector2::new(0.0, 0.0);
        for boid in flock.iter().filter(|b| b.pos.distance_to(&self.pos) < VISUAL_RADIUS && b.pos.distance_to(&self.pos) > 0.0) {
            cohesion_vector.add(&boid.pos);
        }
        cohesion_vector.limit(&MAX_SPEED);
        cohesion_vector
    }

    fn get_separation_vector(&self, flock: &Vec<Boid>) -> Vector2 {
        let mut separation_vector = Vector2::new(0.0, 0.0);
        for boid in flock.iter().filter(|b| b.pos.distance_to(&self.pos) < VISUAL_RADIUS && b.pos.distance_to(&self.pos) > 0.0) {
            let distance = self.pos.distance_to(&boid.pos);
            let mut boid_vector = self.pos.vector_to(&boid.pos);
            boid_vector.x = boid_vector.x / distance;
            boid_vector.y = boid_vector.y / distance;
            separation_vector.sub(&boid_vector);
        }
        separation_vector.limit(&MAX_SPEED);
        separation_vector
    }

    fn get_obstacle_avoidance_vector(&self, obstacles: &Vec<Obstacle>) -> Vector2 {
        let mut avoidance_vector = Vector2::new(0.0, 0.0);
        for obstacle in obstacles.iter().filter(|b| b.pos.distance_to(&self.pos) < VISUAL_RADIUS) {
            let distance = self.pos.distance_to(&obstacle.pos);
            let mut obstacle_vector = self.pos.vector_to(&obstacle.pos);
            obstacle_vector.x = obstacle_vector.x / distance;
            obstacle_vector.y = obstacle_vector.y / distance;
            avoidance_vector.sub(&obstacle_vector);
        }
        avoidance_vector
    }

}

#[derive(Debug)]
pub struct Environment {
    width: f32,
    height: f32,
    pub boids: Vec<Boid>,
    pub obstacles: Vec<Obstacle>,
}

impl Environment {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width: width / 2.0,
            height: height / 2.0,
            boids: vec![],
            obstacles: Environment::initialize_obstacles(width, height),
        }
    }

    pub fn initialize_boids(&mut self, num_boids: usize) {
        for _ in 0..num_boids {
            let pos = Vector2::new(rand::thread_rng().gen_range(-self.width..self.width), rand::thread_rng().gen_range(-self.height..self.height));
            let mut vel= Vector2::new(rand::thread_rng().gen_range(-MAX_SPEED..MAX_SPEED), rand::thread_rng().gen_range(-MAX_SPEED..MAX_SPEED));  
            vel.limit(&MAX_SPEED);
            self.boids.push(Boid::new(pos, vel));
        }
    }
    
    pub fn initialize_obstacles(width: f32, height: f32) -> Vec<Obstacle> {
        let mut obstacles = vec![];
        let tl = Vector2::new(-width/2.0, height/2.0);
        let tr = Vector2::new(width/2.0, height/2.0);
        let bl = Vector2::new(-width/2.0, -height/2.0);
        let br = Vector2::new(width/2.0, -height/2.0);
        for i in (tl.x as i32..tr.x as i32).step_by(50) {
            let top = Vector2::new(i as f32, tl.y);
            let bottom = Vector2::new(i as f32, bl.y);
            obstacles.push(Obstacle::new(top));
            obstacles.push(Obstacle::new(bottom));
        }
        for i in (bl.y as i32..tl.y as i32).step_by(50) {
            let left = Vector2::new(tl.x, i as f32);
            let right = Vector2::new(tr.x, i as f32);
            obstacles.push(Obstacle::new(left));
            obstacles.push(Obstacle::new(right));
        }
        obstacles
    }

    pub fn update(&mut self) {
        let copy = self.boids.clone();
        for boid in self.boids.iter_mut() {
            boid.update(&copy, &self.obstacles, &self.width, &self.height);
        }
    }

}

#[derive(Debug)]
#[derive(Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
impl Vector2 {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }

    fn add(&mut self, other: &Vector2) -> Self {
        self.x += other.x;
        self.y += other.y;
        self.clone()
    }

    fn sub(&mut self, other: &Vector2) -> Self {
        self.x -= other.x;
        self.y -= other.y;
        self.clone()
    }

    fn mult(&mut self, scalar: &f32) -> Self{
        self.x *= scalar;
        self.y *= scalar;
        self.clone()
    }

    fn heading(&self) -> f32 {
        f32::atan2(self.y, self.x)
    }

    fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn limit(&mut self, max: &f32) {
        if self.magnitude() > *max {
            self.x = self.x / self.magnitude() * max;
            self.y = self.y / self.magnitude() * max;
        }
    }

    fn distance_to(&self, other: &Vector2) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    fn vector_to(&self, other: &Vector2) -> Vector2 {
        let heading = f32::atan2(other.y - self.y, other.x - self.x);
        Vector2::new(f32::cos(heading), f32::sin(heading))
    }
}

#[derive(Debug)]
pub struct Obstacle {
    pub pos : Vector2,
}

impl Obstacle {
    pub fn new(pos: Vector2) -> Self {
        Self {
            pos,
        }
    }
}