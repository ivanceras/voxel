
struct Ray{
    direction: Vector, // the direction of the ray
    origin: Point, // the starting point of the ray
    length: f64, // the length of the ray
}


struct FrustumBeam{
    ray: Ray,
    angle: f64,
}


/// conical beam of light which starts with
/// 0 radius from the camera and some
/// radius when touching the screen
/// and finally some radius when it 
/// reaches the maximum length of
/// the ray
struct ConeBeam{
    ray: Ray,
    angle: f64,
}


impl ConeBeam{
    /// the tree that describes the beam at
    /// with at each level of detail
    fn get_voxel_tree()->VoxelTree<bool>{
        VoxelTree::default() 
    }
}


