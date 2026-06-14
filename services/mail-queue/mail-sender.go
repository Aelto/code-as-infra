package main

import "log"

type MailSender struct {
	config WithConfigOptions
}

func MakeMailSender(config WithConfigOptions) MailSender {
	return MailSender{config}
}

func (sender *MailSender) SendMail(request MailRequest) {
	// smtp_username := sender.config.Smtp_username()
	// smtp_password := sender.config.Smtp_password()
	// smtp_address := sender.config.Smtp_address()
	// smtp_port := sender.config.Smtp_port()
	// brevo_api_key := sender.config.Brevo_api_key()

	log.Printf("[MailSender] sending mail from=%q, to=%q", request.from, request.to)
}
