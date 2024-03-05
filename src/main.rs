use glfw;
use gl;
use nalgebra as na;
use rand::prelude::*;

mod shaderutils;
mod modelutils;
mod object;
mod camera;

mod natu;



fn setup(game: &mut natu::Natu) {
    game.load_object("resources/suzanne.obj","monkey","resources/missing.png");
    game.load_object("resources/suzanne.obj", "ico", "resources/missing.png");
    game.load_object("resources/suzanne.obj", "o", "resources/missing.png");
    game.get("ico").scale = 0.1;
    game.get("o").scale = 0.1;
    game.get("monkey").position.z=1.0;
    game.get("monkey").velocity.x=0.1;
    game.camera.position.z = -4.0;
    game.get("ico").position.y = 1.0;
    game.get("ico").velocity.x = 1.0;
    
    for i in 0..100 {
        let key = i.to_string();
        game.load_object("resources/suzanne.obj", key.as_str(), "resources/missing.png");
        let obj = game.get(key.as_str());
        obj.scale = 1.0;
        let mut rng = rand::thread_rng();
        let ran = 100.0;
        obj.position.x = ran*rng.gen::<f32>();
        obj.position.y = ran*rng.gen::<f32>();
        obj.position.z = ran*rng.gen::<f32>();

    }
}

fn tick(game: &mut natu::Natu) {
    //let yaw = 3.0*game.glfw.get_time() as f32;
    //game.get("monkey").yaw = yaw;
    game.get("ico").acceleration = -10.0*game.get("ico").position/game.get("ico").position.magnitude().powf(2.0);
    nbody(game);
    apply_physics(game);
    //game.get("ico").position = game.get("monkey").position + game.get("monkey").acceleration * 0.1
    spectate(game, "5")
}

// 
fn spectate(game: &mut natu::Natu, object_name: &str) {
    {
        game.get(object_name).scale = 1.0;
        let dir = game.get(object_name).velocity.normalize();
        game.camera.position = game.get(object_name).position - game.camera.direction*(10.0 as f32);
    }
    
}

fn nbody(game: &mut natu::Natu) {
    // (Key, Pos, Mass) for each object

    let G = 1000.0;

    let mut accelerations: std::collections::HashMap<String, na::Vector3<f32>> = Default::default();
    
    {
        // Reset all accelerations to 0
        for (key,obj) in &mut game.objects {
            obj.acceleration *= 0.0;
        }

        for (key0,obj0) in &game.objects {
            accelerations.insert(key0.to_string(), na::Vector3::<f32>::new(0.0,0.0,0.0));
            let summer = accelerations.get_mut(key0).unwrap();
    
            for (key1,obj1) in &game.objects {
                if key0 != key1 {
                    // use GMm/r^2 to calculate gravity
                    let m = obj0.mass;
                    let M = obj1.mass;
                    let r2 = (obj0.position - obj1.position).magnitude().powf(2.0);
                    let dir = (obj1.position - obj0.position).normalize();
                    *summer = *summer + dir*G*m*M/r2*(game.delta as f32);
                    //game.get(key0).
                }
            }
        }

        for (key,obj) in &mut game.objects {
            obj.acceleration = obj.acceleration + accelerations.get(key).unwrap();
        }

        
    }
}

// Use delta and object's parameters (r, v, p, etc) to update its position
fn apply_physics(game: &mut natu::Natu) {
    for (_,obj) in &mut game.objects {
        let delta = game.delta;
        obj.velocity += delta as f32 * obj.acceleration;
        obj.position += delta as f32 * obj.velocity;

        // Just for fun: face in direction of movement
        let vel = obj.velocity.normalize();
        
        obj.pitch = (-vel.y).asin();
        obj.yaw = vel.x.atan2(vel.z);
        println!("{:?}", obj.yaw);
    }
    
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
