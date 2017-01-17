use voxeltree::VoxelTree;
use object::Object;
use voxeltree::Transformation;


struct World{ 
    objects: Vec<(Object, Location, Transformation)>
}



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

    /// add an object to the scene
    /// location, transformation
    /// the voxel tree will have to be called again to update the tree
    fn add_object(&mut self, obj: Object, loc: &Location, trans: &Transformation){
        self.objects.push((obj, loc.clone(), trans.clone() ));    
    }
}


#[derive(Clone)]
struct Location{
    position: Vec<u64>, // position at each level
}

impl Location{

    fn get_lod(&self) -> usize{
        self.position.len()
    }

    fn get_voxel_tree(&self) -> VoxelTree<bool>{
        VoxelTree::default()
    }
}
