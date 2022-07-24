enum LogLevel: Int {
	case trace, debug, info = 4, warning, error, fatal, unknown = 42

	init(_ msg: String) {
		switch msg.split(separator: ":")[0] {
		case "[TRC]": self = .trace
		case "[DBG]": self = .debug
		case "[INF]": self = .info
		case "[WRN]": self = .warning
		case "[ERR]": self = .error
		case "[FTL]": self = .fatal
		default: self = .unknown
		}
	}

	func shortFormat(message msg: String) -> String {
		"\(rawValue):\(msg)"
	}
}
