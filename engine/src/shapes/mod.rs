pub mod traits;
pub mod triangle;
pub mod rectangle;
pub mod sphere;

pub use traits::RenderableShape;
pub use triangle::Triangle;
pub use rectangle::Rectangle;
pub use sphere::Sphere;