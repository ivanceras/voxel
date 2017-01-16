# Physically based, dynamic, unlimited detail sparse voxel rendering

Another one of my developer itch which explores on the idea of using
voxels on rendering 3D objects

## Introduction
Voxel has been a long discussion of the next generation of
doing graphics due to its simplicity and pleasant characteristics
that ables to describe volume and Level of detail automatically


## Technique
 A beam of light from the camera to a certain distance will be projected
 to the worl space.
 The beam can be of conical or pyramidal in shape.
 
 The beam will be presented as VoxelTree which has an optimized method
 for testing interesections to the objects in the world space.

 Objects that gets intersected in the world space will be determined 
 if there is a need for additional Level of detail. Additional level
 of detail will be streamed directly from the object file at a certain 
 level of detail

Large worlds are subdivided into voxeltrees as needed

### Compact sparse Voxel
Compact sparse voxel is a way to store data into the disk
This is done by ignoring the children of the bitset which has
zero values, which means, no further detail is in the location of the voxel


### Dynamic desctruction
Desctruction affects voxels at a certail Level of detail and mipmap
of LOD upwards will be recomputed.
Subtle changes in the higher LODs will be adjusted by distribution function
which will still reflect to the object with lower LOD

