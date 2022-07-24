import Foundation

func splitOnNewlines(_ poem: String) -> [String] {
	poem.components(separatedBy: "\n")
}

func firstLetter(_ line: String) -> Character {
	line.first ?? "_"
}

func capitalize(_ phrase: String) -> String {
	phrase.split(separator: " ").map { word in
		(word.first ?? "_").uppercased() + word.dropFirst().lowercased()
	}.joined(separator: " ")
}

func trimFromEnd(_ line: String) -> String {
	line.replacingOccurrences(of: "\\s+$", with: "", options: .regularExpression)
}

func lastLetter(_ line: String) -> Character {
	line.last ?? "_"
}

func backDoorPassword(_ phrase: String) -> String {
	capitalize(phrase) + ", please"
}

func ithLetter(_ line: String, i: Int) -> Character {
	i < line.count ? line[line.index(line.startIndex, offsetBy: i)] : " "
}

func secretRoomPassword(_ phrase: String) -> String {
	phrase.uppercased() + "!"
}
