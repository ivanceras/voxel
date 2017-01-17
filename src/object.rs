use voxeltree::VoxelTree;

pub struct Object{
    file: String
}

impl Object{
    
    fn load_voxel_tree(lod: u8) -> VoxelTree<bool>{
        VoxelTree::default() 
    }
}
