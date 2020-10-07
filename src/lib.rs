/*!
This crate provides a bitmatrix based on [bitvec](https://docs.rs/bitvec), akin to
[bit-matrix](https://docs.rs/bit-matrix) which is based on [bit-vec](https://docs.rs/bit-vec).

Access can be done through the `Index<usize>`, `Index<(usize, usize)>` and
`IndexMut<usize>` traits. As the `IndexMut` trait is incompatible with proxies,
a `set` method is provided. Row and cell iterators are also provided.

If the `serde_support` feature is enabled, this type implements serde's `Serialize`
and `Deserialize` traits.
*/


#[cfg(test)]
mod tests;

use std::{
	fmt,
	ops::{Index, IndexMut}
};

use bitvec::{
	order::Lsb0,
	slice::{BitSlice, ChunksExact, ChunksExactMut, Iter, IterMut},
	boxed::BitBox,
	vec::BitVec,
};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};


/// A matrix of bits.
///
/// Access can be done through the `Index<usize>`, `Index<(usize, usize)>` and
/// `IndexMut<usize>` traits. As the `IndexMut` trait is incompatible with proxies,
/// a `set` method is also provided.
///
/// If the `serde_support` feature is enabled, this type implements serde's `Serialize`
/// and `Deserialize` traits.
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BitMatrix {
	storage: BitBox<Lsb0, usize>,
	height: usize,
	width: usize,
}


impl BitMatrix {
	/// Create a BitMatrix with the given size. All bits are initialized to `false`.
	///
	/// ```
	/// # use bitmatrix::BitMatrix;
	/// let matrix = BitMatrix::new(5, 10);
	/// ```
	pub fn new(height: usize, width: usize) -> Self {
		let size = height * width;
		let mut vec = BitVec::with_capacity(size);
		vec.resize(size, false);

		Self {
			storage: vec.into_boxed_bitslice(),
			height,
			width
		}
	}


	/// Get the matrix height.
	pub fn height(&self) -> usize {
		self.height
	}

	/// Get the matrix width.
	pub fn width(&self) -> usize {
		self.width
	}


	fn row_ix(&self, i: usize) -> usize {
		i * self.width
	}


	/// Iterate over all bits in the matrix.
	/// ```
	/// # use bitmatrix::BitMatrix;
	/// let mut matrix = BitMatrix::new(2, 2);
	/// matrix.set((0,1), true);
	/// matrix.set((1,0), true);
	/// let mut iter = matrix.iter();
	/// assert_eq!(iter.next(), Some(&false));
	/// assert_eq!(iter.next(), Some(&true));
	/// assert_eq!(iter.next(), Some(&true));
	/// assert_eq!(iter.next(), Some(&false));
	/// assert_eq!(iter.next(), None);
	/// ```
	pub fn iter(&self) -> Iter<Lsb0, usize> {
		self.storage.iter()
	}


	/// Iterate over all bits in the matrix.
	/// ```
	/// # use bitmatrix::BitMatrix;
	/// let mut matrix = BitMatrix::new(2, 2);
	/// let mut iter = matrix.iter_mut();
	/// if let Some(bit) = iter.next() {
	/// 	bit.set(true);
	/// }
	/// assert_eq!(matrix[(0,0)], true);
	/// ```
	pub fn iter_mut(&mut self) -> IterMut<Lsb0, usize> {
		self.storage.iter_mut()
	}


	/// An iterator to the matrix rows.
	/// Returns an iterator over BitSlices.
	pub fn rows(&self) -> ChunksExact<Lsb0, usize> {
		self.storage.chunks_exact(self.width)
	}

	/// A mutable iterator to the matrix rows.
	/// Returns a mutable iterator over BitSlices.
	pub fn rows_mut(&mut self) -> ChunksExactMut<Lsb0, usize> {
		self.storage.chunks_exact_mut(self.width)
	}


	/// Set a given bit in the matrix.
	/// This method is necessary because `IndexMut` is incompatible with proxies.
	/// ```
	/// # use bitmatrix::BitMatrix;
	/// let mut matrix = BitMatrix::new(3, 11);
	/// matrix.set((1,2), true);
	/// assert_eq!(matrix[(1,2)], true);
	/// ```
	/// # Panics
	/// Panics if `i` or `j` are out of bounds.
	pub fn set(&mut self, (i, j): (usize, usize), value: bool) {
		self[i].set(j, value);
	}


	/// Set all bits in the matrix.
	/// ```
	/// use std::convert::identity;
	/// # use bitmatrix::BitMatrix;
	/// let mut matrix = BitMatrix::new(3, 11);
	/// matrix.set_all(true);
	/// assert!(matrix.iter().copied().all(identity));
	/// ```
	pub fn set_all(&mut self, value: bool) {
		self.storage.set_all(value);
	}
}


impl Index<usize> for BitMatrix {
	type Output = BitSlice;

	fn index(&self, index: usize) -> &Self::Output {
		let begin = self.row_ix(index);
		let end = self.row_ix(index + 1);

		&self.storage[begin .. end]
	}
}


impl IndexMut<usize> for BitMatrix {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		let begin = self.row_ix(index);
		let end = self.row_ix(index + 1);

		&mut self.storage[begin .. end]
	}
}


impl Index<(usize, usize)> for BitMatrix {
	type Output = bool;

	fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
		&self[i][j]
	}
}


impl fmt::Debug for BitMatrix {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for row in self.rows() {
			for item in row.into_iter() {
				write!(f, "{}", *item as u8)?;
			}

			writeln!(f)?;
		}

		Ok(())
	}
}
