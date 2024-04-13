package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Cell struct {
	value int
	x     int
	y     int
}

type Move struct {
	cell1 Cell
	cell2 Cell
}

func main() {
	reader := bufio.NewReader(os.Stdin)
	// fmt.Println("first_level")
	fmt.Println("nqxjkerlxkoxggjaxnxlxndifqpkjkvh")
	for {
		inputLine, _ := reader.ReadString('\n')
		inputs := strings.Fields(inputLine)
		width, _ := strconv.Atoi(inputs[0])
		height, _ := strconv.Atoi(inputs[1])

		gameMap := make([][]int, height)
		for i := range gameMap {
			gameMap[i] = make([]int, width)
		}

		for i := 0; i < height; i++ {
			inputs, _ := reader.ReadString('\n')
			cellValues := strings.Fields(inputs)
			for j := 0; j < width; j++ {
				x, _ := strconv.Atoi(cellValues[j])
				gameMap[i][j] = x
			}
		}

		// print initial game map
		for _, row := range gameMap {
			// fmt.Fprintln(os.Stderr, row)
			// output for debugging purposes in array format
			fmt.Fprintln(os.Stderr, "{", strings.Trim(strings.Join(strings.Fields(fmt.Sprint(row)), ", "), "[]"), "}")
		}

		solveSteps := computeSteps(gameMap)
		for _, step := range solveSteps {
			fmt.Println(step)
		}
	}
}

func computeSteps(gameMap [][]int) []string {
	var initialViableCells []Cell
	for y, row := range gameMap {
		for x, cell := range row {
			if cell != 0 {
				initialViableCells = append(initialViableCells, Cell{cell, x, y})
			}
		}
	}

	for range initialViableCells {
		gameMapCopy := make([][]int, len(gameMap))
		for i := range gameMap {
			gameMapCopy[i] = make([]int, len(gameMap[i]))
			copy(gameMapCopy[i], gameMap[i])
		}

		gameResult, completionSteps := getGameResult(gameMapCopy)
		if gameResult {
			return completionSteps
		}
	}

	panic("No solution found")
}

func getGameResult(gameMap [][]int) (bool, []string) {
	var completionSteps []string
	var viableCells []Cell
	for y, row := range gameMap {
		for x, cell := range row {
			if cell != 0 {
				viableCells = append(viableCells, Cell{cell, x, y})
			}
		}
	}

	if len(viableCells) == 1 {
		return false, completionSteps
	}

	possibleMoves := getPossibleMoves(viableCells)

	if len(possibleMoves) == 0 {
		return false, completionSteps
	}

	for _, move := range possibleMoves {
		moveDirection := ""
		if move.cell1.x == move.cell2.x {
			if move.cell1.y > move.cell2.y {
				moveDirection = "U"
			} else {
				moveDirection = "D"
			}
		} else {
			if move.cell1.x > move.cell2.x {
				moveDirection = "L"
			} else {
				moveDirection = "R"
			}
		}

		for _, moveSign := range []string{"+", "-"} {
			gameMapCopy := make([][]int, len(gameMap))
			for i := range gameMap {
				gameMapCopy[i] = make([]int, len(gameMap[i]))
				copy(gameMapCopy[i], gameMap[i])
			}
			completionSteps = []string{}
			completionSteps = append(completionSteps, fmt.Sprintf("%d %d %s %s", move.cell1.x, move.cell1.y, moveDirection, moveSign))

			newCellValue := 0
			if moveSign == "+" {
				newCellValue = move.cell1.value + move.cell2.value
			} else {
				newCellValue = abs(move.cell2.value - move.cell1.value)
			}
			gameMapCopy[move.cell2.y][move.cell2.x] = newCellValue
			gameMapCopy[move.cell1.y][move.cell1.x] = 0

			gameResult, gameResultSteps := getGameResult(gameMapCopy)

			if len(gameResultSteps) > 0 {
				completionSteps = append(completionSteps, gameResultSteps...)
			}

			if gameResult {
				return true, completionSteps
			}

			isGameMapEmpty := true
			for _, row := range gameMapCopy {
				for _, cell := range row {
					if cell != 0 {
						isGameMapEmpty = false
						break
					}
				}
				if !isGameMapEmpty {
					break
				}
			}

			if isGameMapEmpty {
				return true, completionSteps
			}
		}
	}

	return false, completionSteps
}

func getPossibleMoves(viableCells []Cell) []Move {
	var possibleMoves []Move
	for _, cell1 := range viableCells {
		for _, cell2 := range viableCells {
			if cell1.x == cell2.x && cell1.y == cell2.y {
				continue
			}

			xDiff := abs(cell1.x - cell2.x)
			yDiff := abs(cell1.y - cell2.y)
			if xDiff != cell1.value && yDiff != cell1.value {
				continue
			}

			if xDiff > 0 && yDiff > 0 {
				continue
			}

			alreadyExists := false
			for _, move := range possibleMoves {
				if cell1.x == move.cell2.x && cell1.y == move.cell2.y && cell2.x == move.cell1.x && cell2.y == move.cell1.y {
					alreadyExists = true
					break
				}
			}

			if !alreadyExists {
				possibleMoves = append(possibleMoves, Move{cell1, cell2})
			}
		}
	}

	return possibleMoves
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}
