use std::collections::HashSet;
use std::sync::{Arc, Barrier};
use std::thread;
use crate::component::identity::id_generator::IdGenerator;

/* -----------------------------------------------------------
	 1. sequential: 1, 2, 3 …
 ----------------------------------------------------------- */
 #[test]
 fn sequential_ids_start_at_one() {
	  let g = IdGenerator::default();
	  assert_eq!(g.next_id(), 1);
	  assert_eq!(g.next_id(), 2);
	  assert_eq!(g.next_id(), 3);
 }



 /* -----------------------------------------------------------
	 3. concurrent uniqueness: spawn many threads, collect ids
 ----------------------------------------------------------- */
 #[test]
 fn unique_across_threads() {
	  const THREADS: usize = 8;
	  const IDS_PER_THREAD: usize = 10_000;

	  let g = Arc::new(IdGenerator::default());
	  let mut handles = Vec::with_capacity(THREADS);
	  let barrier = Arc::new(Barrier::new(THREADS));

	  for _ in 0..THREADS {
			let g = Arc::clone(&g);
			let b = Arc::clone(&barrier);
			handles.push(thread::spawn(move || {
				 // synchronise start so threads race properly
				 b.wait();
				 (0..IDS_PER_THREAD).map(|_| g.next_id()).collect::<Vec<u32>>()
			}));
	  }

	  let mut all: Vec<u32> = handles
			.into_iter()
			.flat_map(|h| h.join().expect("thread panicked"))
			.collect();

	  // Check total count
	  assert_eq!(all.len(), THREADS * IDS_PER_THREAD);

	  // Check uniqueness
	  let set: HashSet<u32> = all.drain(..).collect();
	  assert_eq!(set.len(), THREADS * IDS_PER_THREAD);
 }
