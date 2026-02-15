use alloc::vec::Vec;
use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::int::triangulation::IntTriangulation as CoreIntTriangulation;

use super::IntTriangulationIndex;

/// Flattened representation of an integer triangulation for FFI usage.
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct FlatIntTriangulation {
    pub flat_points: Vec<i32>,
    pub indices: Vec<IntTriangulationIndex>,
}

impl FlatIntTriangulation {
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
    pub fn set_triangulation(
        &mut self,
        triangulation: &CoreIntTriangulation<IntTriangulationIndex>,
    ) {
        self.clear_and_reserve(triangulation.points.len(), triangulation.indices.len() / 3);
        self.push_triangulation(triangulation);
    }

    /// Appends a triangulation without clearing existing contents.
    ///
    /// The caller must ensure enough capacity has been reserved.
    #[inline]
    pub fn push_triangulation(
        &mut self,
        triangulation: &CoreIntTriangulation<IntTriangulationIndex>,
    ) {
        for point in &triangulation.points {
            self.flat_points.push(point.x);
            self.flat_points.push(point.y);
        }

        self.indices.extend(triangulation.indices.iter().copied());
    }

    /// Converts the stored data back into a core triangulation.
    #[inline]
    pub fn to_triangulation(&self) -> CoreIntTriangulation<IntTriangulationIndex> {
        let mut triangulation = CoreIntTriangulation::default();
        self.fill_triangulation(&mut triangulation);
        triangulation
    }

    /// Fills an existing triangulation with the stored values.
    #[inline]
    pub fn fill_triangulation(
        &self,
        triangulation: &mut CoreIntTriangulation<IntTriangulationIndex>,
    ) {
        triangulation.points.clear();
        triangulation.points.reserve(self.flat_points.len() / 2);

        for coords in self.flat_points.chunks_exact(2) {
            triangulation
                .points
                .push(IntPoint::new(coords[0], coords[1]));
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

impl From<&CoreIntTriangulation<IntTriangulationIndex>> for FlatIntTriangulation {
    #[inline]
    fn from(value: &CoreIntTriangulation<IntTriangulationIndex>) -> Self {
        let mut flat =
            FlatIntTriangulation::with_capacity(value.points.len(), value.indices.len() / 3);
        flat.push_triangulation(value);
        flat
    }
}
