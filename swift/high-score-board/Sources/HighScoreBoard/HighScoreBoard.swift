func newScoreBoard() -> [String: Int] {
	[String: Int]()
}

func addPlayer(_ scores: inout [String: Int], _ name: String, _ score: Int = 0) {
	scores[name] = score
}

func removePlayer(_ scores: inout [String: Int], _ name: String) {
	scores.removeValue(forKey: name)
}

func resetScore(_ scores: inout [String: Int], _ name: String) {
	scores[name] != nil ? scores[name] = 0 : ()
}

func updateScore(_ scores: inout [String: Int], _ name: String, _ delta: Int) {
	scores[name] = scores[name, default: 0] + delta
}

func orderByPlayers(_ scores: [String: Int]) -> [(String, Int)] {
	scores.sorted { lhs, rhs in lhs.0 < rhs.0 }
}

func orderByScores(_ scores: [String: Int]) -> [(String, Int)] {
	scores.sorted { lhs, rhs in lhs.1 > rhs.1 }
}
