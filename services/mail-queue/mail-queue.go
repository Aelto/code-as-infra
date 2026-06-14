package main

import (
	"encoding/json"
	"fmt"
	"html"
	"log"
	"net/http"
)

func main() {
	fmt.Println("Hello, World!")

	open_server()
}

func open_server() {
	sender := MakeMailSender(MakeEnvConfigProvider())

	http.HandleFunc("GET /mails", func(w http.ResponseWriter, r *http.Request) {
		log.Printf("received request on %q", html.EscapeString(r.URL.Path))

	})

	http.HandleFunc("POST /mails", func(w http.ResponseWriter, r *http.Request) {
		log.Printf("received request on %q", html.EscapeString(r.URL.Path))

		type Body struct {
			From  string   `json:"from"`
			To    []string `json:"to"`
			Title string   `json:"title"`
			Body  string   `json:"body"`
		}

		form, err := deserializeBody[Body](r)
		if err != nil {
			http.Error(w, "invalid data", http.StatusConflict)
			return
		}

		sender.SendMail(makeMailRequest(form.From, form.To, form.Title, form.Body))
	})

	log.Fatal(http.ListenAndServe(":3000", nil))
}

func deserializeBody[DES any](r *http.Request) (DES, error) {
	decoder := json.NewDecoder(r.Body)

	var form DES
	err := decoder.Decode(&form)

	if err != nil {
		return form, err
	}

	return form, nil
}
