
struct Frustum{
    near_plane: f64, //distance from the camera to the projection plane
    far_plane: f64, //the maximum length of ray to travel
    side_angle: f64, // the angle in radians from the center of camera to 1 side
    elevation_angle: f64, // the angle in radians from the center of camera and the top
}


///       camera direction
///             |
///             v
///
///    .--------+--------,  <--- far plane
///     \       |       /
///      \      |z     /
///       \     |     /
///        \    | x  /
///         \---+---/   <--- near plane
///          \  |  /
///           \ | /
///            \|/
///             '   <--- camera
/// 
impl Frustum{

    
    /// the frustum in camera space
    /// the the far plane extends to the z coordinate
    /// the left and the right is the x coordinate
    /// the top and bottom of the screen is the y coordinate
    /// the camera is at 0,0,0
    ///       
    ///   .---. (x,z)
    ///   |  /
    ///   | /(r)
    ///   |/
    ///   '
    fn is_inside(self, x:f64, y:f64, z:f64) -> bool{
        if z >= self.near_plane && z < self.far_plane{
            //compute the angle of point (x,z) and if it is 
            //lesser than the side angle
            // tan(t) = x/z
            let angle_xz = (x/z).atan();
            if angle_xz <= self.side_angle{
                let angle_yz = (y/z).atan();   
                if angle_yz <= self.elevation_angle{
                    return true;
                }
            }
        } 
        false
    }
}
