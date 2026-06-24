package main

import (
	"crypto/tls"
	"fmt"
	"log"
	"net/smtp"
	"strings"
)

type MailSender struct {
	config WithConfigOptions
}

func MakeMailSender(config WithConfigOptions) MailSender {
	fmt.Printf("SMTP server: address=%s, port=%s\n", config.Smtp_address(), config.Smtp_port())

	return MailSender{config}
}

func (sender *MailSender) SendMail(request MailRequest) {
	smtp_username := sender.config.Smtp_username()
	smtp_password := sender.config.Smtp_password()
	smtp_address := sender.config.Smtp_address()
	smtp_address_with_port := smtp_address + ":" + sender.config.Smtp_port()

	log.Printf("sending mail from=%q, to=%q", request.from, request.to)

	auth := smtp.PlainAuth("", smtp_username, smtp_password, smtp_address)
	tlsconfig := &tls.Config{
		InsecureSkipVerify: true,
		ServerName:         smtp_address,
	}

	conn, err := tls.Dial("tcp", smtp_address_with_port, tlsconfig)
	if err != nil {
		log.Panic(err)
	}

	client, err := smtp.NewClient(conn, smtp_address)
	if err != nil {
		log.Panic(err)
	}

	if err = client.Auth(auth); err != nil {
		log.Panic(err)
	}

	if err = client.Mail(request.from); err != nil {
		log.Panic(err)
	}

	if err = client.Rcpt(strings.Join(request.to, ",")); err != nil {
		log.Panic(err)
	}

	writer, err := client.Data()
	if err != nil {
		log.Panic(err)
	}

	message := []byte(
		"From: " + request.from + "\r\n" +
			"To: " + strings.Join(request.to, ",") + "\r\n" +
			"Subject: " + request.title + "\r\n" +
			"\r\n" +
			request.body +
			"\r\n")

	_, err = writer.Write(message)
	if err != nil {
		log.Panic(err)
	}

	err = writer.Close()
	if err != nil {
		log.Panic(err)
	}
}
