pub mod boids;

use boids::*;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    env: boids::Environment,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let mut env = Environment::new(1920.0, 1080.0);
    env.initialize_boids(50);
    Model { _window, env: env }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.env.update();
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(BLACK);

    // Draw the boid area
    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(1920.0, 1080.0)
        .color(rgb(0.2, 0.2, 0.2));


    // Draw the obstacles
    /*for obstacle in _model.env.obstacles.iter() {
        draw.ellipse()
            .x_y(obstacle.pos.x, obstacle.pos.y)
            .w_h(45.0, 45.0)
            .color(RED);
    }*/

    // Draw the boids.
    for boid in _model.env.boids.iter() {
        let start_point = pt2(boid.pos.x, boid.pos.y);
        let end_point = pt2(boid.pos.x + boid.vel.x * 50.0, boid.pos.y + boid.vel.y * 50.0);
        draw.arrow().start(start_point).end(end_point).color(WHITE).weight(2.0);
    }

    draw.to_frame(app, &frame).unwrap();
}