struct MaybePizza {
	let diameter: Double?
	let slices: Int?
	var sliceArea: Double? {
		guard let d = diameter, let s = slices, d >= 0, s >= 1 else { return nil }
		return (Double.pi * d * d / 4) / Double(s)
	}
}

extension MaybePizza: Comparable {
	static func < (lhs: MaybePizza, rhs: MaybePizza) -> Bool {
		switch (lhs.sliceArea, rhs.sliceArea) {
		case (nil, nil): return false
		case (_?, nil): return false
		case (nil, _?): return true
		case let (l?, r?): return l < r
		}
	}

	static func == (lhs: MaybePizza, rhs: MaybePizza) -> Bool {
		switch (lhs.sliceArea, rhs.sliceArea) {
		case (nil, nil): return true
		case (_?, nil): return false
		case (nil, _?): return false
		case let (l?, r?): return l == r
		}
	}
}

func sliceSize(diameter: Double?, slices: Int?) -> Double? {
	MaybePizza(diameter: diameter, slices: slices).sliceArea
}

func biggestSlice(
	diameterA: String, slicesA: String,
	diameterB: String, slicesB: String
) -> String {
	let lhs = MaybePizza(diameter: Double(diameterA), slices: Int(slicesA))
	let rhs = MaybePizza(diameter: Double(diameterB), slices: Int(slicesB))
	if lhs == rhs { return "Neither slice is bigger" }
	return "Slice \(lhs > rhs ? "A" : "B") is bigger"
}
