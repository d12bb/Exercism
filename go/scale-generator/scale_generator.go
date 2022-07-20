package scale

var (
	notes     = []string{"A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"}
	intervals = map[rune]int{
		'm': 1,
		'M': 2,
		'A': 3,
	}
	alternates = [][]string{
		{"A#", "Bb"},
		{"C#", "Db"},
		{"D#", "Eb"},
		{"F#", "Gb"},
		{"G#", "Ab"},
	}
)

func Scale(tonic, interval string) []string {
	bs := []string{"F", "bb", "Eb", "g", "d", "Db"}
	useAlternate := false
	for _, b := range bs {
		if tonic == b {
			useAlternate = true
		}
	}

	if tonic[0] > 96 && tonic[0] < 123 { // if is lowercase
		tonic = string(tonic[0]-32) + tonic[1:]
	}

	if interval == "" { // chromatic
		interval = "mmmmmmmmmmmm"
	}
	interval = interval[:len(interval)-1]

	scale := append(make([]string, 0, len(interval)), tonic)
	current := findNote(tonic)
	for _, ival := range interval {
		current = ((current+intervals[ival])%12 + 12) % 12
		scale = append(scale, notes[current])
	}

	if useAlternate {
		for i, v := range scale {
			if i == 0 {
				continue
			}
			scale[i] = getAlternate(v)
		}
	}

	return scale
}

func findNote(note string) (idx int) {
	alt := getAlternate(note)
	for i, n := range notes {
		if note == n || alt == n {
			return i
		}
	}
	panic("wtf is this note???")
}

func getAlternate(note string) string {
	if len(note) == 1 {
		return note
	}
	for _, alts := range alternates {
		if alts[0] == note {
			return alts[1]
		}
		if alts[1] == note {
			return alts[0]
		}
	}
	return note
}
