pub struct CircularBuffer<T> {
	head: usize,
	tail: usize,
	capacity: usize,
	content: Vec<Option<T>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
	EmptyBuffer,
	FullBuffer,
}

impl<T> CircularBuffer<T> {
	pub fn new(capacity: usize) -> Self {
		let mut buf = CircularBuffer {
			head: 0,
			tail: 0,
			capacity,
			content: vec![],
		};
		for _ in 0..capacity {
			buf.content.push(None);
		}
		buf
	}

	pub fn write(&mut self, data: T) -> Result<(), Error> {
		match self.content[self.head] {
			Some(_) => Err(Error::FullBuffer),
			None => {
				self.content[self.head] = Some(data);
				self.head = self.next(self.head);
				Ok(())
			}
		}
	}

	pub fn read(&mut self) -> Result<T, Error> {
		let elem = self.content[self.tail].take();
		match elem {
			Some(data) => {
				self.tail = self.next(self.tail);
				Ok(data)
			}
			None => Err(Error::EmptyBuffer),
		}
	}

	pub fn clear(&mut self) {
		for i in 0..self.capacity {
			self.content[i] = None
		}
		self.head = 0;
		self.tail = 0;
	}

	pub fn overwrite(&mut self, data: T) {
		if self.content[self.head].is_none() {
			self.write(data).unwrap();
		} else {
			let idx = self.tail;
			self.content[idx] = Some(data);
			self.head = idx;
			self.tail = self.next(self.tail);
		}
	}

	fn next(&self, idx: usize) -> usize {
		let mut out = idx + 1;
		if out >= self.capacity {
			out = 0;
		}
		out
	}
}
