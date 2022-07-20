package letter

import (
	"strings"
	"sync"
)

type FreqMap map[rune]int

// Out of boredom I tested multiple approches:
//  - Using one hashmap per worker, unifiyng after: 112573 ns/op
//  - Using Mutex and chans:                        1413655 ns/op
//  - Using Mutex and WaitGroup:                    1413273 ns/op
//
// As expected, Mutexes are slow..
//
// Bench on MacBook Pro (M1 Pro) 2021

// # Concurrently using one hashmap per worker
func ConcurrentFrequency(l []string) FreqMap {
	freqMap := FreqMap{}

	c := make(chan FreqMap, len(l))
	for _, part := range l {
		go func(s string) {
			c <- Frequency(s)
		}(part)
	}

	for i := 0; i < len(l); i++ {
		part := <-c
		for k, v := range part {
			freqMap[k] += v
		}
	}

	return freqMap
}

type mFreqMap struct {
	freqMap FreqMap
	sync.Mutex
}

// Concurrently using mutex and waitgroup
func ConcurrentFrequencyGroupedMutexWaitGroup(l []string) FreqMap {
	freqMap := mFreqMap{freqMap: FreqMap{}}
	var wg sync.WaitGroup

	for _, part := range l {
		wg.Add(1)
		go func(s string) {
			for _, g := range strings.Split(s, "\n") {
				freqMap.Lock()
				freqMap.freqMap['\n']++
				for _, r := range g {
					freqMap.freqMap[r]++
				}
				freqMap.Unlock()
			}
			wg.Done()
		}(part)
	}
	wg.Wait()
	freqMap.freqMap['\n'] -= len(l)

	return freqMap.freqMap
}

// Concurrently using mutex and channel
func ConcurrentFrequencyMutexChan(l []string) FreqMap {
	freqMap := mFreqMap{freqMap: FreqMap{}}
	jobs := len(l)
	c := make(chan bool, jobs)

	for _, part := range l {
		go func(s string, done chan<- bool) {
			for _, r := range s {
				freqMap.Lock()
				freqMap.freqMap[r]++
				freqMap.Unlock()
			}
			done <- true
		}(part, c)
	}

	for i := 0; i < jobs; i++ {
		<-c
	}
	return freqMap.freqMap
}

// Frequency counts the frequency of each rune in a given text and returns this
// data as a FreqMap.
func Frequency(s string) FreqMap {
	m := FreqMap{}
	for _, r := range s {
		m[r]++
	}
	return m
}
