func remainingMinutesInOven(elapsedMinutes: Int, expectedMinutesInOven: Int = 40) -> Int {
	expectedMinutesInOven - elapsedMinutes
}

func preparationTimeInMinutes(layers: String...) -> Int {
	layers.count * 2
}

func quantities(layers: String...) -> (noodles: Int, sauce: Double) {
	layers.reduce((noodles: 0, sauce: 0.0)) { r, l in
		switch l {
		case "noodles": return (noodles: r.0 + 3, sauce: r.1)
		case "sauce": return (noodles: r.0, sauce: r.1 + 0.2)
		default: return r
		}
	}
}

func toOz(_ q: inout (noodles: Int, sauce: Double)) {
	q.sauce = q.sauce * 33.814
}

// use five nested functions? wtf???
func redWine(layers: String...) -> Bool {
	layers.reduce(0) { r, l in
		switch l {
		case "mozzarella", "ricotta", "béchamel": return r - 1
		case "meat", "sauce": return r + 1
		default: return r
		}
	} >= 0
}
