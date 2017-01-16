use voxeltree::VoxelTree;


struct World; 


impl World{
    
    /// get the voxel tree at certain lod
    /// of the world, relative to the camera location
    /// world voxel has a minimum lod loaded
    /// objects near camera will be loaded with higher lod
    /// objects farther the camera will require lower lod
    /// zoomed-in areas of the world will also load further higher
    /// LOD of object along the zoomed-in rays
    fn get_voxel_tree() -> VoxelTree<bool>{
        VoxelTree::default()
    }
}
