use crate::coords::error::Error;
use crate::coords::error::Error::IndexOutOfBounds;
use itertools::iproduct;
use rayon::prelude::*;
use std::collections::BTreeSet;
use std::fmt;
use std::fmt::Debug;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Ord, PartialOrd)]
pub struct OctantIndex {
    pub level: u32,
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl fmt::Display for OctantIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OctantIndex(level: {}, x: {}, y: {}, z: {})",
            self.level, self.x, self.y, self.z
        )
    }
}

impl OctantIndex {
    pub fn new(level: u32, x: u64, y: u64, z: u64) -> Result<Self, Error> {
        let maximum_index = 2_u64.pow(level) - 1;
        if x > maximum_index || y > maximum_index || z > maximum_index {
            return Err(IndexOutOfBounds {
                level,
                maximum_index,
                x,
                y,
                z,
            });
        }

        Ok(Self { level, x, y, z })
    }

    pub fn new_unchecked(level: u32, x: u64, y: u64, z: u64) -> Self {
        Self { level, x, y, z }
    }

    pub fn origin() -> Self {
        Self {
            level: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    pub fn morton_index(&self) -> Result<u64, Error> {
        if self.x > u32::MAX as u64 || self.y > u32::MAX as u64 || self.z > u32::MAX as u64 {
            return Err(Error::IndexTooLarge);
        }

        let code =
            crate::coords::morton::morton_encode(self.x as u32, self.y as u32, self.z as u32);
        Ok(code)
    }

    pub fn get_child_base_octant(&self) -> Self {
        Self {
            level: self.level + 1,
            x: self.x * 2,
            y: self.y * 2,
            z: self.z * 2,
        }
    }

    pub fn has_parent(&self) -> bool {
        self.level > 0
    }

    pub fn get_parent(&self) -> Option<Self> {
        if !self.has_parent() {
            return None;
        }

        Some(Self {
            level: self.level - 1,
            x: self.x / 2,
            y: self.y / 2,
            z: self.z / 2,
        })
    }

    /// Returns all ancestor octants of this octant, including itself.
    ///
    /// An ancestor is any octant at a higher level (lower level number) in the octree
    /// hierarchy that contains this octant. The method traverses up the tree from the
    /// current octant to the root, collecting all octants along the path.
    ///
    /// # Returns
    ///
    /// A `BTreeSet` containing all ancestors, including:
    /// - The current octant itself
    /// - Its immediate parent (if any)
    /// - All ancestors up to the root octant (level 0)
    ///
    /// The returned set is ordered by the natural ordering of `OctantIndex`
    /// (level, then x, y, z coordinates).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ecoord_core::octree::OctantIndex;
    /// let octant = OctantIndex::new(2, 3, 2, 1).unwrap();
    /// let ancestors = octant.get_ancestors();
    ///
    /// // The set will contain:
    /// // - OctantIndex { level: 2, x: 3, y: 2, z: 1 } (self)
    /// // - OctantIndex { level: 1, x: 1, y: 1, z: 0 } (parent)
    /// // - OctantIndex { level: 0, x: 0, y: 0, z: 0 } (root)
    /// assert_eq!(ancestors.len(), 3);
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(log n) where n is the maximum possible level, since we traverse up
    /// the tree hierarchy level by level until reaching the root.
    pub fn get_ancestors(&self) -> BTreeSet<Self> {
        let mut ancestors = BTreeSet::new();
        let mut current = *self;

        ancestors.insert(current);

        while let Some(parent) = current.get_parent() {
            ancestors.insert(parent);
            current = parent;
        }

        ancestors
    }

    fn get_descendent_base_octant(&self, level_offset: u32) -> Self {
        let scale_factor = 2u64.pow(level_offset);

        Self {
            level: self.level + level_offset,
            x: self.x * scale_factor,
            y: self.y * scale_factor,
            z: self.z * scale_factor,
        }
    }

    pub fn get_descendents(&self, level_offset: u32) -> Vec<Self> {
        let descendent_base = self.get_descendent_base_octant(level_offset);

        let maximum_index = 2_u64.pow(level_offset);
        let num_indices = (maximum_index * maximum_index * maximum_index) as usize;
        let mut indices = Vec::with_capacity(num_indices);

        for (current_x, current_y, current_z) in
            iproduct!(0..maximum_index, 0..maximum_index, 0..maximum_index)
        {
            let octant_index = OctantIndex::new(
                descendent_base.level,
                descendent_base.x + current_x,
                descendent_base.y + current_y,
                descendent_base.z + current_z,
            )
            .expect("should work");
            indices.push(octant_index);
        }

        indices
    }

    pub fn get_children(&self) -> [Self; 8] {
        let child_base = self.get_child_base_octant();

        [
            child_base,
            Self {
                level: child_base.level,
                x: child_base.x + 1,
                y: child_base.y,
                z: child_base.z,
            },
            Self {
                level: child_base.level,
                x: child_base.x,
                y: child_base.y + 1,
                z: child_base.z,
            },
            Self {
                level: child_base.level,
                x: child_base.x + 1,
                y: child_base.y + 1,
                z: child_base.z,
            },
            Self {
                level: child_base.level,
                x: child_base.x,
                y: child_base.y,
                z: child_base.z + 1,
            },
            Self {
                level: child_base.level,
                x: child_base.x + 1,
                y: child_base.y,
                z: child_base.z + 1,
            },
            Self {
                level: child_base.level,
                x: child_base.x,
                y: child_base.y + 1,
                z: child_base.z + 1,
            },
            Self {
                level: child_base.level,
                x: child_base.x + 1,
                y: child_base.y + 1,
                z: child_base.z + 1,
            },
        ]
    }
}

pub trait VecOctantIndexExt {
    fn sort_by_morton_indices(&self) -> Result<Vec<(OctantIndex, u64)>, Error>;
}

impl VecOctantIndexExt for Vec<OctantIndex> {
    fn sort_by_morton_indices(&self) -> Result<Vec<(OctantIndex, u64)>, Error> {
        let mut indices: Vec<(OctantIndex, u64)> = self
            .iter()
            .map(|octant_index| {
                let morton_index = octant_index.morton_index()?;
                Ok((*octant_index, morton_index))
            })
            .collect::<Result<Vec<_>, Error>>()?;

        indices.sort_by_key(|k| (k.0.level, k.1));
        Ok(indices)
    }
}

#[cfg(test)]
mod octree_index_test {
    use crate::coords::error::Error::IndexOutOfBounds;
    use crate::octree::OctantIndex;

    #[test]
    fn test_basic_index_construction() {
        let index = OctantIndex::new(2, 1, 3, 0);

        assert!(index.is_ok());
    }

    #[test]
    fn test_index_out_of_bounds() {
        let index = OctantIndex::new(0, 1, 0, 0);
        let expected = Err(IndexOutOfBounds {
            level: 0,
            maximum_index: 0,
            x: 1,
            y: 0,
            z: 0,
        });

        assert_eq!(index, expected);
    }

    #[test]
    fn test_parent() {
        let index = OctantIndex::new(2, 3, 0, 0).expect("should work");
        let parent = index.get_parent().unwrap();

        assert_eq!(parent, OctantIndex::new(1, 1, 0, 0).expect("should work"));
    }

    #[test]
    fn test_children() {
        let index = OctantIndex::new(1, 1, 0, 1).expect("should work");
        let children = index.get_children();

        println!("{children:?}");
        //assert_eq!(parent, OctantIndex::new(1, 1, 0, 0).expect("should work"));
    }

    #[test]
    fn test_descendents() {
        let index = OctantIndex::new(1, 1, 0, 1).expect("should work");
        let children = index.get_descendents(2);

        println!("{children:?}");
        //assert_eq!(parent, OctantIndex::new(1, 1, 0, 0).expect("should work"));
    }
}
