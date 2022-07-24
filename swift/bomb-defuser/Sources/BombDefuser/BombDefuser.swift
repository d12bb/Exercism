typealias Wires = (String, String, String)

let flip: (Wires) -> Wires = { ($0.1, $0.0, $0.2) }
let rotate: (Wires) -> Wires = { ($0.1, $0.2, $0.0) }

func makeShuffle(
	flipper: @escaping (Wires) -> Wires,
	rotator: @escaping (Wires) -> Wires
) -> (UInt8, Wires) -> Wires {
	{
		var wires = $1
		var str = String($0, radix: 2)
		while str.count < 8 {
			str = "0" + str
		}

		// exercise description not matching tests here
		str.reversed().forEach { c in
			if c == "1" {
				wires = rotator(wires)
			} else {
				wires = flipper(wires)
			}
		}
		return wires
	}
}
