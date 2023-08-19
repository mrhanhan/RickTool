package main

import "fmt"

//export Hello
func Hello() {
	fmt.Print("Hello For Go")
}

func main() {
	Hello()
}