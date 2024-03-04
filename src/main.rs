use glfw;
use gl;

mod shaderutils;
mod modelutils;
mod object;
mod camera;

mod natu;

fn setup(game: &mut natu::Natu) {
    //game.load_object("resources/suzanne.obj","monkey","resources/missing.png");
    game.load_object("resources/icosphere.obj", "ico", "");
    game.load_object("resources/icosphere.obj", "o", "");
    game.get("ico").scale = 0.1;
    game.get("o").scale = 0.1;
    game.camera.position.z = -4.0;
    game.get("ico").position.y = 1.0;
    game.get("ico").velocity.x = 6.0;
}

fn tick(game: &mut natu::Natu) {
    //let yaw = 3.0*game.glfw.get_time() as f32;
    //game.get("monkey").yaw = yaw;
    game.get("ico").acceleration = -10.0*game.get("ico").position/game.get("ico").position.magnitude().powf(2.0);
    apply_physics(game, "ico");
    //game.get("ico").position = game.get("monkey").position + game.get("monkey").acceleration * 0.1
}


// Use delta and object's parameters (r, v, p, etc) to update its position
fn apply_physics(game: &mut natu::Natu, name: &str) {
    let delta = game.delta;
    let object = game.get(name);
    object.velocity += delta as f32 * object.acceleration;
    object.position += delta as f32 * object.velocity;
}

fn main() {
    let mut game = natu::Natu::init();

    // Load objects
    setup(&mut game);

    // Begin render loop
    while !game.window.should_close() {
        // Handle per-frame events such as physics
        tick(&mut game);
        game.update();

        // Enforce framerate
        game.pause_until_frame();
    }
}
