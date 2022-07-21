func canIBuy(vehicle v: String, price p: Double, monthlyBudget b: Double) -> String {
	let monthly = p / 60
	switch b - monthly {
	case 0...: return "Yes! I'm getting a " + v
	case -100 ..< 0: return "I'll have to be frugal if I want a " + v
	case ..<(-100): return "Darn! No " + v + " for me"
	default: return "error"
	}
}

func licenseType(numberOfWheels n: Int) -> String {
	switch n {
	case 0: return "We do not issue licenses for those types of vehicles"
	case 2 ... 3: return "You will need a motorcycle license for your vehicle"
	case 4 | 6: return "You will need an automobile license for your vehicle"
	case 18: return "You will need a commercial trucking license for your vehicle"
	default: return "error"
	}
}

func registrationFee(msrp: Int, yearsOld a: Int) -> Int {
	guard a < 10 else { return 25 }

	let price = msrp > 25000 ? msrp : 25000
	return (price - a * price / 10) / 100
}
