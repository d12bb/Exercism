package tournament

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"strings"
)

type tournament struct {
	games     []*game
	teams     map[uint8]*team
	teamnames map[string]uint8
	lastid    uint8
}

type game struct {
	t1, t2 string
	result string
}

type team struct {
	id     uint8
	name   string
	wins   uint8
	draws  uint8
	losses uint8
	score  uint8 // cache score for sort
}

func newTournament() *tournament {
	return &tournament{teams: map[uint8]*team{}, teamnames: map[string]uint8{}}
}

func (t *tournament) newOrGetTeam(name string) *team {
	if team, ok := t.teams[t.teamnames[name]]; ok {
		return team
	}

	t.lastid++
	team := &team{id: t.lastid, name: name}
	t.teams[t.lastid] = team
	t.teamnames[name] = t.lastid
	return team
}

func (t *tournament) sort() []uint8 {
	ids := make([]uint8, len(t.teams))
	i := 0
	for _, team := range t.teams {
		ids[i] = team.id
		i++
		team.cache_score() // no need to calculate it every time
	}

	// insertion sort, q&d
	for i := range ids {
		for j := i; j > 0 && t.teams[ids[j-1]].score < t.teams[ids[j]].score; j-- {
			ids[j], ids[j-1] = ids[j-1], ids[j]
		}
	}

	// alphabetical sub sort for same score
	for i := range ids {
		score := t.teams[ids[i]].score
		j := i
		for j < len(ids) && score == t.teams[ids[j]].score {
			j++
		}
		j--

		sub := ids[i : j+1]
		for si := range sub {
			for sj := si; sj > 0 && t.teams[sub[sj-1]].name > t.teams[sub[sj]].name; sj-- {
				sub[sj], sub[sj-1] = sub[sj-1], sub[sj]
			}
		}
		i = j
	}

	return ids
}

func (t *tournament) format() string {
	res := "Team                           | MP |  W |  D |  L |  P\n"
	for _, id := range t.sort() {
		team, ok := t.teams[id]
		if !ok {
			panic("")
		}
		res += fmt.Sprintf("%-30s | %2d | %2d | %2d | %2d | %2d\n", team.name, team.games(), team.wins, team.draws, team.losses, team.points())
	}
	return res
}

func newGame(s string) (*game, error) {
	ss := strings.Split(s, ";")
	if len(ss) != 3 {
		return nil, errors.New("malformed input")
	}
	return &game{t1: ss[0], t2: ss[1], result: ss[2]}, nil
}

func (g *game) parse(tour *tournament) error {
	t1 := tour.newOrGetTeam(g.t1)
	t2 := tour.newOrGetTeam(g.t2)

	switch g.result {
	case "win":
		t1.wins++
		t2.losses++
	case "draw":
		t1.draws++
		t2.draws++
	case "loss":
		t1.losses++
		t2.wins++
	default:
		return errors.New("malformed input")
	}

	return nil
}

func (t *team) games() uint8 {
	return t.wins + t.draws + t.losses
}

func (t *team) points() uint8 {
	return 3*t.wins + t.draws
}

func (t *team) cache_score() {
	t.score = t.points()
}

func Tally(reader io.Reader, writer io.Writer) error {
	t := newTournament()

	s := bufio.NewScanner(reader)
	for s.Scan() {
		line := s.Text()
		if strings.TrimSpace(line) == "" || strings.HasPrefix(line, "#") {
			continue
		}

		game, err := newGame(line)
		if err != nil {
			return err
		}
		t.games = append(t.games, game)
	}
	if err := s.Err(); err != nil {
		return err
	}

	for _, game := range t.games {
		err := game.parse(t)
		if err != nil {
			return err
		}
	}

	_, err := fmt.Fprint(writer, t.format())
	if err != nil {
		return err
	}

	return nil
}