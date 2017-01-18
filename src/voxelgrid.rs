/// a sequential
/// voxel grid of 1's and 0's 
/// arranged in morton z code
/// testing intersection at this level
/// is merely `and`ing the `u64`s 
/// 
/// a heirarchial mipmap of voxeltree
/// can be contructed based on this voxel grid
pub struct VoxelGrid{
    bitset: Vec<u64>,
}
