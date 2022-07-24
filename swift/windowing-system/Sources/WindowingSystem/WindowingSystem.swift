struct Size {
	var width = 80
	var height = 60

	mutating func resize(newWidth: Int, newHeight: Int) {
		width = newWidth
		height = newHeight
	}
}

struct Position {
	var x = 0
	var y = 0

	mutating func moveTo(newX: Int, newY: Int) {
		x = newX
		y = newY
	}
}

class Window {
	let screenSize = Size(width: 800, height: 600)

	var title = "New Window"
	var size = Size()
	var position = Position()
	var contents: String?

	init() {}

	init(title: String, size: Size, position: Position, contents: String?) {
		self.title = title
		self.size = size
		self.position = position
		self.contents = contents
	}

	func resize(to newSize: Size) {
		var newSize = newSize
		if newSize.width < 1 { newSize.width = 1 }
		if newSize.height < 1 { newSize.height = 1 }
		if position.x + newSize.width > screenSize.width { newSize.width = screenSize.width - position.x }
		if position.y + newSize.height > screenSize.height { newSize.height = screenSize.height - position.y }
		size = newSize
	}

	func move(to newPos: Position) {
		var newPos = newPos
		if newPos.x < 0 { newPos.x = 0 }
		if newPos.y < 0 { newPos.y = 0 }
		if newPos.x + size.width > screenSize.width { newPos.x = screenSize.width - size.width }
		if newPos.y + size.height > screenSize.height { newPos.y = screenSize.height - size.height }
		position = newPos
	}

	func update(title: String) {
		self.title = title
	}

	func update(text: String?) {
		contents = text
	}

	func display() -> String {
		return """
			\(title)
			Position: (\(position.x), \(position.y)), Size: (\(size.width) x \(size.height))
			\(contents ?? "[This window intentionally left blank]")\n
			"""
	}
}

let mainWindow = Window(
	title: "Main Window",
	size: Size(width: 400, height: 300),
	position: Position(x: 100, y: 100),
	contents: "This is the main window"
)
