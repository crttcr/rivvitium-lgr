
use std::fmt::{self, Debug, Display, Formatter};
use crate::model::ir::byte_row::{ByteRow, ByteRowBounds};

pub struct NVStrings {
	pairs:     Vec<(String, String)>,
}

impl NVStrings {
	pub fn new(pairs: Vec<(String, String)>) -> Self {
		NVStrings {pairs}
	}
}

impl Display for NVStrings {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let count = self.pairs.len();
        write!(f, "NVStrings ({} pair{})", count, if count == 1 { "" } else { "s" })
    }
}

impl Debug for NVStrings {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let total      = self.pairs.len();
        let show_count = 2;                   // How many to show before ellipsis
        f.write_str("NVStrings{")?;
        if total != 0
        {
            if total <= show_count {
                for (i, (k, v)) in self.pairs.iter().enumerate() {
                    write!(f, "(\"{}\", \"{}\")", k, v)?;
                    if i + 1 != total { f.write_str(", ")?; }
                }
            } else {
                for (k, v) in &self.pairs[..show_count] {      // Show first `show_count` and then ellipsis
                    write!(f, "(\"{}\", \"{}\"), ", k, v)?;
                }
                let remaining = total - show_count;
                write!(f, "… and {} more", remaining)?;
            }
        }
        f.write_str("}")
    }
}

#[cfg(test)]
mod tests {
	use super::NVStrings;

	#[test]
	fn debug_empty() {
		let pairs = Vec::new();
		let nv    = NVStrings{pairs};
		assert_eq!(format!("{nv:?}"), "NVStrings{}");
	}

	#[test]
	fn debug_few() {
		let nv = NVStrings {
		pairs: vec![ ("a".into(), "1".into()), ("b".into(), "2".into())]};
		let dbg = format!("{nv:?}");
		assert_eq!(dbg, r#"NVStrings{("a", "1"), ("b", "2")}"#);
	}

	#[test]
	fn debug_many() {
		let nv = NVStrings {
			pairs: vec![
				("a".into(), "1".into()),
				("b".into(), "2".into()),
				("c".into(), "3".into()),
				("d".into(), "4".into()),
				],
			};
		let dbg = format!("{nv:?}");
		// First two pairs shown, then ellipsis and count of remaining (4 - 2 = 2)
		assert_eq!(
				dbg,
				r#"NVStrings{("a", "1"), ("b", "2"), … and 2 more}"#
				);
	}

	#[test]
	fn display_zero() {
		let nv = NVStrings { pairs: Vec::new() };
		assert_eq!(format!("{nv}"), "NVStrings (0 pairs)");
	}

	#[test]
	fn display_one() {
	let nv = NVStrings {
	pairs: vec![("key".into(), "val".into())],
	};
	assert_eq!(format!("{nv}"), "NVStrings (1 pair)");
	}

	#[test]
	fn display_many() {
	let nv = NVStrings {
	pairs: vec![
	("x".into(), "y".into()),
	("u".into(), "v".into()),
	("p".into(), "q".into()),
	],
	};
	assert_eq!(format!("{nv}"), "NVStrings (3 pairs)");
	}
}
