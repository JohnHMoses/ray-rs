
#[derive(Copy, Clone)]
pub enum Token {
    Unknown,                    // Placeholder
    Eofsym,                     // End Of File
    SbtRaytracer,

    Ident,                      // Ident (gets Enclosed In Quotes In Trace Files)
    Scalar,                     // Scalar Values
    Symtrue,
    Symfalse,

    Lparen, Rparen,             // Punctuation
    Lbrace, Rbrace,
    Comma,
    Equals,
    Semicolon,

    Camera,                     // Camera Primitive
    PointLight,                // Lights
    DirectionalLight,
    AmbientLight,

    ConstantAttenuationCoeff, // Terms Affecting The Intensity Dropoff
    LinearAttenuationCoeff,   // Of Point Lights (see The Pointlight 
    QuadraticAttenuationCoeff,// Class)

    Sphere,                     // Primitives
    Box,
    Square,
    Cylinder,
    Cone,
    Trimesh,  

    Position, Viewdir,          // Keywords Affecting Primitives
    Updir, Aspectratio,
    Fov,
    Color,
    Direction,
    Capped,
    Height,
    BottomRadius,
    TopRadius,
    Quaternian,                 // ???

    Polypoints, Normals,        // Keywords Affecting Polygons
    Materials, Faces,
    Gennormals,

    Translate, Scale,           // Transforms
    Rotate, Transform,

    Material,                   // Material Settings
    Emissive, Ambient, 
    Specular, Reflective,
    Diffuse, Transmissive,
    Shininess, Index,
    Name,
    Map,
}