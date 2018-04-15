//! Algorithms needed for distance and penetration depth computation.

pub use self::cso_point::CSOPoint;
#[cfg(feature = "dim2")]
pub use self::epa2::EPA2;
#[cfg(feature = "dim3")]
pub use self::epa3::EPA3;
pub use self::simplex::Simplex;
#[cfg(feature = "dim2")]
pub use self::voronoi_simplex2::VoronoiSimplex2;
#[cfg(feature = "dim3")]
pub use self::voronoi_simplex3::VoronoiSimplex3;

mod cso_point;
#[cfg(feature = "dim2")]
pub mod epa2;
#[cfg(feature = "dim3")]
pub mod epa3;
pub mod gjk;
mod simplex;
#[cfg(feature = "dim2")]
mod voronoi_simplex2;
#[cfg(feature = "dim3")]
mod voronoi_simplex3;
// pub mod minkowski_sampling;
