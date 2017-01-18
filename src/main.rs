mod voxeltree;
mod vector;
mod camera;
mod world;
mod object;
mod point;
mod frustum;

fn main() {
    println!("Hello, world!");
}

/// the coordinate reference based on reference
enum Space{
    Camera, // what the camera sees
    World, // the scene of the world, all objects will be abiding the world space
    // all objects will be relative to the world space
    // including camera location, rotation, elavation and roll
    Object, // the local space of the detailed object
}

