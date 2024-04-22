package main

import (
	"fmt"
	"log"
	"os"
	"runtime/pprof"
	"testing"
)

func TestLevel12(t *testing.T) {
	gameMap := [][]int{
		{0, 0, 0, 3, 0, 0, 0, 0},
		{4, 0, 0, 0, 1, 0, 0, 3},
		{0, 0, 0, 4, 0, 1, 3, 0},
		{0, 0, 0, 4, 1, 1, 0, 0},
		{0, 0, 2, 0, 1, 0, 0, 0},
	}

	// profile
	f, err := os.Create("test_level12.prof")
	if err != nil {
		log.Fatal(err)
	}
	pprof.StartCPUProfile(f)
	defer pprof.StopCPUProfile()

	solveSteps := computeSteps(gameMap)
	for _, step := range solveSteps {
		fmt.Println(step)
	}
}
