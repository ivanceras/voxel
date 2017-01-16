use voxeltree::VoxelTree;

struct Object{
    file: String
}

impl Object{
    
    fn load_voxel_tree(lod: u8) -> VoxelTree<bool>{
        VoxelTree::default() 
    }
}
