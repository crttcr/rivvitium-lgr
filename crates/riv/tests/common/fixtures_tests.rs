
use crate::common::fixtures::TestAtoms;

#[test]
pub fn test_nv_pairs_fixture() {
	let atoms    = TestAtoms::nv_pairs();
	println!("{:?}", atoms.first().unwrap());
}

#[test]
pub fn test_start_end_vec() {
	let atoms    = TestAtoms::start_end_vec();
	println!("{:?}", atoms.first().unwrap());
}
