package main

import (
	"fmt"
)

func main() {
	initGame()
	// TODO: randomize first player, socket prog
	for winner() == -1 {
		printGame()
		fmt.Printf("Enter move: ")
		var move int
		fmt.Scanf("%d\n", &move)
		i := (move - 1) / 3
		j := (move - 1) % 3
		if i >= 0 && i < 3 && j >= 0 && j < 3 && game[i][j] == -1 {
			play(i, j, 0)
		} else {
			fmt.Printf("Invalid move \n\n")
		}

	}
}
