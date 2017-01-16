
struct Ray{
    direction: Vector,
    origin: Point,
}


struct BeamFrustum{
    ray: Ray,
    angle: f64,
}


/// conical beam of light which starts with
/// 0 radius from the camera and some
/// radius when touching the screen
/// and finally some radius when it 
/// reaches the maximum length of
/// the ray
struct Beam{
    ray: Ray,
    angle: f64,
}


impl Beam{
    /// the tree that descibes the beam at
    /// with at each level of detail
    fn get_voxel_tree()->VoxelTree<bool>{
        VoxelTree::default() 
    }
}
