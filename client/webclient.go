package main

import (
	"encoding/json"
	"html/template"
	"log"
	"net/http"
	"fmt"
)

type TemplateData struct {
	CurrentCount int
}

type Response struct {
	CurrentCount int `json:"current_count"`
}

func main() {
    RunClient()
}

func RunClient() {
	http.HandleFunc("/", webPage)
	err := http.ListenAndServe(":8000", nil)
	if err != nil {
		log.Panic(err)
	}
}

func webPage(w http.ResponseWriter, r *http.Request) {
	resp, err := http.Get("http://localhost:8080/current")
	if err != nil {
	    fmt.Println("Error! {}", err)
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}

	decoder := json.NewDecoder(resp.Body)
	val := Response{}
	err = decoder.Decode(&val)
	current := val.CurrentCount
	if err != nil {
		log.Fatal(err)
	}

	data := TemplateData{
		CurrentCount: current,
	}
	fmt.Println("Serving template")
	tmpl, err := template.ParseFiles("webclient.html")
	if err != nil {
		log.Panic(err)
	}
	err = tmpl.Execute(w, data)
	if err != nil {
		log.Panic(err)
	}
}
