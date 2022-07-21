func protectSecret(_ secret: String, withPassword password: String) -> (String) -> String {
	return { (pass: String) -> String in
		guard pass == password else { return "Sorry. No hidden secrets here." }
		return secret
	}
}

func generateCombination(forRoom room: Int, usingFunction f: (Int) -> Int) -> (Int, Int, Int) {
	let n1 = f(room)
	let n2 = f(n1)
	let n3 = f(n2)
	return (n1, n2, n3)
}
