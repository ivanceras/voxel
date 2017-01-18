use voxeltree::VoxelTree;

pub struct Point{
    x: f64,
    y: f64,
    z: f64
}


impl Point{
    
    /// The voxel tree will be mostly filled with zeroes
    fn get_voxel_tree()->VoxelTree<bool>{
        VoxelTree::default() 
    }

    /// the location of the point using 
    /// an array of u64.
    /// with the value describes the location of the point
    /// relative to the parent location of the bitset in the cube
    /// there is only 1 bit that is set at each value
    /// at each level of detail 
    fn get_location_tree() -> Vec<u64> {
       vec![] 
    }
}
