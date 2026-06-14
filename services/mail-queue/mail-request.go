package main

import (
	"time"
)

type MailRequest struct {
	from  string
	to    string
	title string
	body  string

	created_at time.Time
}

func makeMailRequest(from, to, title, body string) MailRequest {
	created_at := time.Now()

	return MailRequest{from, to, title, body, created_at}
}

func (request *MailRequest) send() {

}
