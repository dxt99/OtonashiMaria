package main

import "fmt"

var game [3][3]int //-1: empty, 0: player1, 1: player2

// initializes board
func initGame() {
	for i := 0; i < 3; i++ {
		for j := 0; j < 3; j++ {
			game[i][j] = -1
		}
	}
}

// checks current winner of game, -1 if none
func winner() int {
	for i := 0; i < 3; i++ {
		// horizontal
		if game[i][0] == game[i][1] && game[i][0] == game[i][2] && game[i][0] != -1 {
			return game[i][0]
		}
	}
	for j := 0; j < 3; j++ {
		// vertical
		if game[0][j] == game[1][j] && game[0][j] == game[2][j] && game[0][j] != -1 {
			return game[0][j]
		}
	}
	// diagonals
	if game[0][0] == game[1][1] && game[0][0] == game[2][2] && game[0][0] != -1 {
		return game[0][0]
	}
	if game[0][2] == game[1][1] && game[0][2] == game[2][0] && game[0][2] != -1 {
		return game[0][0]
	}
	return -1
}

// p -> player number (0/1)
func play(i int, j int, p int) {
	if winner() != -1 {
		return
	}
	game[i][j] = p
}

// prints current board
func printGame() {
	for i := 0; i < 3; i++ {
		for j := 0; j < 3; j++ {
			if game[i][j] == -1 {
				fmt.Printf("%d ", i*3+j+1)
			} else if game[i][j] == 0 {
				fmt.Printf("X ")
			} else if game[i][j] == 1 {
				fmt.Printf("O ")
			}
		}
		fmt.Println()
	}
}

// checks if board is full
func isFull() bool {
	for i := 0; i < 3; i++ {
		for j := 0; j < 3; j++ {
			if game[i][j] == -1 {
				return false
			}
		}
	}
	return true
}
