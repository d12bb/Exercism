package strand

func ToRNA(dna string) string {
	rna := make([]rune, 0, len(dna))
	for _, r := range dna {
		switch r {
		case 'G':
			rna = append(rna, 'C')
		case 'C':
			rna = append(rna, 'G')
		case 'T':
			rna = append(rna, 'A')
		case 'A':
			rna = append(rna, 'U')
		}
	}
	return string(rna)
}
