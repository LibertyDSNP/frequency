//! # Suffix Generator
//!
//! `suffix_generator` provides a `SuffixGenerator` struct to generate unique suffix sequences for a given range
//! and seed, excluding already used suffixes.

use rand::{rngs::SmallRng, Rng, SeedableRng};
use sp_std::vec::Vec;

/// A generator for unique suffix sequences.
///
/// Given a min, max range, and a seed, generates unique suffix sequences by excluding
/// already used suffixes.
pub struct SuffixGenerator {
	min: u32,
	max: u32,
	rng: SmallRng,
}

impl SuffixGenerator {
	/// Creates a new `SuffixGenerator` instance with the specified min, max range and seed.
	///
	/// # Example
	///
	/// ```
	/// use crate::suffix::SuffixGenerator;
	///
	/// let min = 100;
	/// let max = 150;
	/// let seed = 12345;
	///
	/// let suffix_generator = SuffixGenerator::new(min, max, seed);
	/// ```
	pub fn new(min: u32, max: u32, seed: u64) -> Self {
		let rng = SmallRng::seed_from_u64(seed);
		Self { min, max, rng }
	}

	/// Generate a unique, shuffled suffix iterator starting from the specified index.
	///
	/// # Arguments
	///
	/// * `start_index` - The starting index for generating the shuffled sequence.
	///
	/// # Returns
	///
	/// An iterator over the unique, shuffled sequence of suffixes, starting from the specified index.
	///
	/// # Examples
	///
	/// ```
	/// use frequency_handles::SuffixGenerator;
	///
	/// let min = 100;
	/// let max = 150;
	/// let seed = 12345;
	///
	/// let mut suffix_generator = SuffixGenerator::new(min, max, seed);
	///
	/// let start_index = 10;
	/// let sequence: Vec<u32> = suffix_generator.suffix_iter(start_index).collect();
	/// println!("{:?}", sequence);
	/// ```
	///
	/// This will output a unique, shuffled sequence of suffix
	pub fn suffix_iter(&mut self) -> impl Iterator<Item = usize> + '_ {
		let mut indices: Vec<usize> = (self.min..=self.max).collect();
		(self.min..=self.max).rev().map(move |i| {
			let j = self.rng.gen_range(0..=i);
			indices.swap(i as usize, j as usize);
			indices[i as usize]
		})
	}
}