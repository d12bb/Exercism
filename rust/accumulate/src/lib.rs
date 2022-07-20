pub fn map<T1, T2, F>(input: Vec<T1>, mut f: F) -> Vec<T2>
where
	F: FnMut(T1) -> T2,
{
	let mut out = vec![];
	for item in input {
		out.push(f(item));
	}
	out
}
