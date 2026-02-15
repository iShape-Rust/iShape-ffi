use alloc::vec::Vec;
use i_triangle::float::triangulation::Triangulation as CoreTriangulation;
use i_triangle::i_overlay::i_float::float::point::FloatPoint;

use super::IntTriangulationIndex;

type Float32Point = FloatPoint<f32>;
type Triangulation32 = CoreTriangulation<Float32Point, IntTriangulationIndex>;

/// Flattened representation of a single-precision triangulation for FFI usage.
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct FlatF32Triangulation {
    pub flat_points: Vec<f32>,
    pub indices: Vec<IntTriangulationIndex>,
}

impl FlatF32Triangulation {
    /// Constructs an empty buffer reserving enough capacity for the provided counts.
    #[inline]
    pub fn with_capacity(points: usize, triangles: usize) -> Self {
        Self {
            flat_points: Vec::with_capacity(points * 2),
            indices: Vec::with_capacity(triangles * 3),
        }
    }

    /// Returns `true` if no triangles are stored.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.flat_points.is_empty()
    }

    /// Clears the stored points and indices while keeping the allocated capacity.
    #[inline]
    pub fn clear(&mut self) {
        self.flat_points.clear();
        self.indices.clear();
    }

    /// Copies the triangulation data into the flat buffers, resizing as needed.
    #[inline]
    pub fn set_triangulation(&mut self, triangulation: &Triangulation32) {
        self.clear_and_reserve(triangulation.points.len(), triangulation.indices.len() / 3);
        self.push_triangulation(triangulation);
    }

    /// Appends a triangulation without clearing existing contents.
    ///
    /// The caller must ensure enough capacity has been reserved.
    #[inline]
    pub fn push_triangulation(&mut self, triangulation: &Triangulation32) {
        for point in &triangulation.points {
            self.flat_points.push(point.x);
            self.flat_points.push(point.y);
        }

        self.indices.extend(triangulation.indices.iter().copied());
    }

    /// Converts the stored data back into a core triangulation.
    #[inline]
    pub fn to_triangulation(&self) -> Triangulation32 {
        let mut triangulation = Triangulation32::with_capacity(self.flat_points.len() / 2);
        self.fill_triangulation(&mut triangulation);
        triangulation
    }

    /// Fills an existing triangulation with the stored values.
    #[inline]
    pub fn fill_triangulation(&self, triangulation: &mut Triangulation32) {
        triangulation.points.clear();
        triangulation.points.reserve(self.flat_points.len() / 2);

        for coords in self.flat_points.chunks_exact(2) {
            triangulation
                .points
                .push(FloatPoint::new(coords[0], coords[1]));
        }

        triangulation.indices.clear();
        triangulation.indices.reserve(self.indices.len());
        triangulation.indices.extend(self.indices.iter().copied());
    }

    #[inline]
    fn clear_and_reserve(&mut self, points: usize, triangles: usize) {
        self.clear();

        self.flat_points.reserve(points * 2);
        self.indices.reserve(triangles * 3);
    }
}

impl From<&Triangulation32> for FlatF32Triangulation {
    #[inline]
    fn from(value: &Triangulation32) -> Self {
        let mut flat =
            FlatF32Triangulation::with_capacity(value.points.len(), value.indices.len() / 3);
        flat.push_triangulation(value);
        flat
    }
}
