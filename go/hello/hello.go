package main

import (
	"fmt"
	"log"
	"simonespa/greetings"

	"rsc.io/quote"
)

func main() {
	log.SetPrefix("ERROR: ")
	log.SetFlags(0)

	fmt.Println(quote.Go())
	// Get a greeting message and print it.
	message, err := greetings.Hello("")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(message)
}
