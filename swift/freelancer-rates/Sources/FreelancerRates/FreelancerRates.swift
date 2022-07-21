func dailyRateFrom(hourlyRate: Int) -> Double {
	Double(8 * hourlyRate)
}

func monthlyRateFrom(hourlyRate h: Int, withDiscount d: Double) -> Double {
	((22 * 8 * Double(h)) * (100 - d) / 100).rounded()
}

func workdaysIn(budget b: Double, hourlyRate h: Int, withDiscount d: Double) -> Double {
	(b / monthlyRateFrom(hourlyRate: h, withDiscount: d) * 22).rounded(.down)
}
