pub fn project_xyz_2_uvw(xyz : [f32;3]) -> [f32;3] {
    //orthographic projection where the camera is 30degrees above the xy plane
    let (s30, c30) = ((30.0 as f32)/180.0 * 3.14159).sin_cos();
    let u = xyz[0];
    let v = xyz[1]*s30 + xyz[2]*c30;
    let w = -xyz[1] * c30;
    [u, v, w]
}