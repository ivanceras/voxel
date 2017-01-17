
/// voxel grid of 1's and 0's 
/// arranged in morton z code
/// testing intersection at this level
/// is merely `and`ing the `u64`s 
/// 
pub struct VoxelGrid{
    bitset: Vec<u64>
}
