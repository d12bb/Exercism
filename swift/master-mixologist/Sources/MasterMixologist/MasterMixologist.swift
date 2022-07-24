// no traditional loops needed ;)
// would be more efficient, though

func timeToPrepare(drinks: [String]) -> Double {
	drinks.reduce(0.0) { r, d in
		switch d {
		case "beer", "soda", "water": return r + 0.5
		case "shot": return r + 1
		case "mixed drink": return r + 1.5
		case "fancy drink": return r + 2.5
		case "frozen drink": return r + 3
		default: return r
		}
	}
}

func makeWedges(needed: Int, limes: [String]) -> Int {
	limes.reduce((0, needed)) { r, l in
		guard r.1 > 0 else { return r }
		switch l {
		case "small": return (r.0 + 1, r.1 - 6)
		case "medium": return (r.0 + 1, r.1 - 8)
		case "large": return (r.0 + 1, r.1 - 10)
		default: return r
		}
	}.0
}

func finishShift(minutesLeft: Int, remainingOrders: [[String]]) -> [[String]] {
	var minutesLeft = Double(minutesLeft)

	return remainingOrders.filter { o in
		guard minutesLeft > 0 else { return true }
		minutesLeft -= timeToPrepare(drinks: o)
		return false
	}
}

typealias Log = (first: String, last: String, total: Int)?
func orderTracker(orders: [(drink: String, time: String)]) -> (beer: Log, soda: Log) {
	func track(_ drink: Log, _ time: String) -> Log {
		(drink == nil ? time : drink!.first, time, drink == nil ? 1 : drink!.total + 1)
	}

	return orders.reduce((beer: nil, soda: nil)) { r, order in
		switch order.drink {
		case "beer": return (track(r.beer, order.time), r.1)
		case "soda": return (r.0, track(r.soda, order.time))
		default: return r
		}
	}
}
