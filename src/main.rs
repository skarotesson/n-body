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
    game.load_object("resources/suzanne.obj","monkey","resources/rock.png");
    game.get("monkey").mass = 0.0;
    game.get("monkey").scale = 0.500;
    game.get("monkey").position.x = 010.0;
    game.camera.position.z = -4.0;
    
    for i in 0..100 {
        let key = i.to_string();
        game.load_object("resources/suzanne.obj", key.as_str(), "resources/missing.png");
        let obj = game.get(key.as_str());
        obj.scale = 0.1;
        let mut rng = rand::thread_rng();
        let ran = 10.0;
        obj.position.x = ran*(0.5-rng.gen::<f32>());
        obj.position.y = ran*(0.5-rng.gen::<f32>());
        obj.position.z = ran*(0.5-rng.gen::<f32>());

    }
}

fn tick(game: &mut natu::Natu) {
    nbody(game);
    apply_physics(game);
    spectate(game, "0")
}

// 
fn spectate(game: &mut natu::Natu, object_name: &str) {
    if game.get(object_name).velocity.magnitude() != 0.0 {
        game.camera.position = game.get(object_name).position - game.camera.direction*(10.0 as f32);
    }
}

fn nbody(game: &mut natu::Natu) {
    // (Key, Pos, Mass) for each object

    let G = 10.0;

    let mut accelerations: std::collections::HashMap<String, na::Vector3<f32>> = Default::default();
    
    {
        // Reset all accelerations to 0
        for (key,obj) in &mut game.objects {
            obj.acceleration *= 0.0;
        }

        for (key0,obj0) in &game.objects {
            accelerations.insert(key0.to_string(), na::Vector3::<f32>::new(0.0,0.0,0.0));
            let summer = accelerations.get_mut(key0).unwrap();

            if obj0.mass != 0.0 {
                for (key1,obj1) in &game.objects {
                    if key0 != key1 {
                        // use GMm/r^2 to calculate gravity
                        let m = obj0.mass;
                        let M = obj1.mass;
                        let r2 = (obj0.position - obj1.position).magnitude().powf(2.0);
                        let dir = (obj1.position - obj0.position).normalize();
                        if r2 != 0.0 {
                            *summer = *summer + dir*G*m*M/r2*(game.delta as f32);
                        }
                        
                        //game.get(key0).
                    }
                }
            } else {
                *summer = obj0.acceleration;
            }
        }

        for (key,obj) in &mut game.objects {
            println!("{}: {:?}",key, obj.acceleration);
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
        
        
        if obj.velocity.magnitude() != 0.0 {
            let vel = obj.velocity.normalize();
            obj.pitch = (-vel.y).asin();
            obj.yaw = vel.x.atan2(vel.z);
            println!("{:?}", obj.yaw);
        }
        
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
