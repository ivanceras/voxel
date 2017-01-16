use vector::Vector;

/// the sparse voxel tree that represents a structure
pub struct VoxelTree<T>{
    bitset: u64,
    content: Option<T>,
    children: Vec<VoxelTree<T>>
}

impl <T>Default for VoxelTree<T>{
    fn default()->Self{
        VoxelTree{
            bitset: 0,
            content: None,
            children: vec![]
        }
    }
}


struct Transformation{
    rotation: Vector,
    scale: Vector,
}

struct BoundingBox{
    bound1: Vector,
    bound2: Vector,
}


impl <T>VoxelTree<T>{
    
    /// calculate the intersection of the 2 voxeltree and return 
    /// a voxel tree structure of the intersection
    fn intersection(&self, other: &VoxelTree<T>)->Self{
        VoxelTree::default() 
    }

    /// transform the voxel tree into the a different coordinate space
    fn transform(&self, tr: &Transformation) -> Self{
        VoxelTree::default() 
    }

    /// creates a voxel tree based on the bounding boxes
    fn from_bounding_box(&self, bbox: &BoundingBox)->Self{
        VoxelTree::default()
    }

    /// creates a voxel tree based on the function equation which determines the
    /// shape of the voxel
    fn from_equation<F>(fnc: F)->Self where F: Fn(Vector)->bool{
        VoxelTree::default() 
    }


    /// a compressed format for storing the data into the disk
    /// the data will be stored, voxel with zero bitset
    /// will be discarded from the storage to save space
    /// the arrangement of the bitset is arranged in morton code order
    fn to_compact_sparse_voxel() -> Vec<u64>{
        vec![]
    }

}
