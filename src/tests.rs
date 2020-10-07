use super::*;


#[test]
#[should_panic]
fn test_out_of_bounds_tuple() {
	let matrix = BitMatrix::new(5, 7);
	let _ = matrix[(1,8)];
}


#[test]
#[should_panic]
fn test_out_of_bounds_index() {
	let matrix = BitMatrix::new(5, 7);
	let _ = matrix[5][1];
}


#[test]
#[should_panic]
fn test_out_of_bounds_set() {
	let mut matrix = BitMatrix::new(5, 7);
	matrix.set((5,1), true);
}


#[test]
#[should_panic]
fn test_out_of_bounds_ix_set() {
	let mut matrix = BitMatrix::new(5, 7);
	*matrix[1].get_mut(10).unwrap() = true;
}
