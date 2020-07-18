use std::mem::size_of;

mod private {
	pub trait Sealed {}

	impl<T> Sealed for [T] {}
}

pub trait FindByRef<T: Sized>: private::Sealed {
	fn find_by_ref(&self, to_find: &T) -> Option<usize>;
}

impl <T> FindByRef<T> for [T] {
	fn find_by_ref(&self, to_find: &T) -> Option<usize> {
		let self_memory_range = {
			let first = self.first();
			let last = self.last();
			if first.is_none() || last.is_none() {
				return None;
			}
			(first.unwrap() as * const T as usize)..(last.unwrap() as * const T as usize + 1)
		};
		let to_find_memory = to_find as * const T as usize;
		if !self_memory_range.contains(&to_find_memory) {
			None
		} else {
			Some((to_find_memory - self_memory_range.start) / size_of::<T>())
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::FindByRef;

	#[test]
	fn it_works() {
		let vector: Vec<&str> = vec!["a", "b", "c", "d"];
		let to_find: &&str = vector.get(2).unwrap(); // "c"
		let index: usize = vector.find_by_ref(to_find).unwrap();
		assert_eq!(vector.get(index).unwrap(), to_find);
		assert_eq!(*vector.get(index).unwrap(), "c");
	}
}
