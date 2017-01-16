use vector::Vector;
use voxeltree::VoxelTree;



struct Camera{
    location: Vector,
    direction: Vector,
}

impl Camera{
    
    fn get_voxel_tree() -> VoxelTree<bool>{
       VoxelTree::default() 
    }
}
