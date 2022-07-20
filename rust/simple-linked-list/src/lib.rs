use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
	head: Option<Box<Node<T>>>,
}

struct Node<T> {
	data: T,
	next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
	pub fn new() -> Self {
		Self { head: None }
	}

	pub fn is_empty(&self) -> bool {
		self.head.is_none()
	}

	pub fn len(&self) -> usize {
		let mut len = 0;
		let mut cur = &self.head;
		while let Some(node) = cur {
			len += 1;
			cur = &node.next;
		}
		len
	}

	pub fn push(&mut self, element: T) {
		let node = Box::new(Node {
			data: element,
			next: self.head.take(),
		});
		self.head = Some(node);
	}

	pub fn pop(&mut self) -> Option<T> {
		let head = self.head.take();
		match head {
			Some(node) => {
				self.head = node.next;
				Some(node.data)
			}
			None => None,
		}
	}

	pub fn peek(&self) -> Option<&T> {
		match &self.head {
			Some(head) => Some(&head.data),
			None => None,
		}
	}

	#[must_use]
	pub fn rev(mut self) -> SimpleLinkedList<T> {
		let mut reversed = SimpleLinkedList::new();
		while let Some(elem) = self.pop() {
			reversed.push(elem);
		}
		reversed
	}
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut out = SimpleLinkedList::new();
		for elem in iter {
			out.push(elem);
		}
		out
	}
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
	fn from(mut list: SimpleLinkedList<T>) -> Vec<T> {
		let mut out = vec![];
		while let Some(elem) = list.pop() {
			out.push(elem);
		}
		out.reverse();
		out
	}
}
