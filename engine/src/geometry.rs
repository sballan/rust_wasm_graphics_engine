pub const TRIANGLE_VERTICES: [f32; 9] = [
    0.0,  0.5, 0.0,
   -0.5, -0.5, 0.0,
    0.5, -0.5, 0.0,
];

pub const CUBE_VERTICES: [f32; 24] = [
    // Simple cube with 8 vertices
    -0.5, -0.5, -0.5,  // 0: back-bottom-left
     0.5, -0.5, -0.5,  // 1: back-bottom-right
     0.5,  0.5, -0.5,  // 2: back-top-right
    -0.5,  0.5, -0.5,  // 3: back-top-left
    -0.5, -0.5,  0.5,  // 4: front-bottom-left
     0.5, -0.5,  0.5,  // 5: front-bottom-right
     0.5,  0.5,  0.5,  // 6: front-top-right
    -0.5,  0.5,  0.5,  // 7: front-top-left
];

pub const CUBE_INDICES: [u16; 36] = [
    // Front face (4,5,6,7) - counter-clockwise when viewed from outside
    4, 5, 6,    4, 6, 7,
    // Back face (0,1,2,3) - counter-clockwise when viewed from outside
    0, 2, 1,    0, 3, 2,
    // Left face (0,3,7,4) - counter-clockwise when viewed from outside
    0, 7, 3,    0, 4, 7,
    // Right face (1,5,6,2) - counter-clockwise when viewed from outside
    1, 2, 6,    1, 6, 5,
    // Bottom face (0,4,5,1) - counter-clockwise when viewed from outside
    0, 1, 5,    0, 5, 4,
    // Top face (3,2,6,7) - counter-clockwise when viewed from outside
    3, 6, 2,    3, 7, 6,
];

pub const CUBE_WIREFRAME_INDICES: [u16; 24] = [
    // 12 edges of the cube
    0, 1,   1, 2,   2, 3,   3, 0,  // back face edges
    4, 5,   5, 6,   6, 7,   7, 4,  // front face edges  
    0, 4,   1, 5,   2, 6,   3, 7,  // connecting back to front
];