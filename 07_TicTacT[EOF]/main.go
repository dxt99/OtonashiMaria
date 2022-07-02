package main

import (
	"fmt"
	"net"
	"strconv"
	"time"
)

// ports
var createRoom = 6000
var joinRoom = 6001

func main() {
	initGame()
	// create/join room
	status := -1
	for status == -1 {
		fmt.Printf("Welcome to TicTacTo[EOF]\n")
		fmt.Printf("1. Create Room \n")
		fmt.Printf("2. Join Room\n")
		fmt.Printf(">> ")
		fmt.Scanf("%d\n, &status")
	}
	// establish conn
	var sendPort int
	var listenPort int
	if status == 1 {
		sendPort = joinRoom
		listenPort = createRoom
	} else {
		sendPort = createRoom
		listenPort = joinRoom
	}
	addr := net.UDPAddr{
		Port: listenPort,
		IP:   net.ParseIP("localhost"),
	}
	sendConn, _ := net.Dial("udp", "localhost:"+strconv.Itoa(sendPort))
	listenConn, _ := net.ListenUDP("udp", &addr)
	// randomize start and send sym
	time := time.Now().Unix()
	p := int(time % 2)
	if status == 1 {
		// broadcast starting position, wait for ack
		buf := []byte("0 " + strconv.Itoa(p))
		_, _ = sendConn.Write(buf)
	} else {
		// listen to starting position then send ack

	}
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
