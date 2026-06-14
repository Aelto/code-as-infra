package main

import "os"

type WithConfigOptions interface {
	Smtp_username() string
	Smtp_password() string
	Smtp_address() string
	Smtp_port() string
}

type EnvConfigProvider struct{}

func MakeEnvConfigProvider() EnvConfigProvider {
	return EnvConfigProvider{}
}

func (o EnvConfigProvider) Smtp_username() string {
	return os.Getenv("SMTP_USERNAME")
}

func (o EnvConfigProvider) Smtp_password() string {
	return os.Getenv("SMTP_PASSWORD")
}

func (o EnvConfigProvider) Smtp_address() string {
	return os.Getenv("SMTP_ADDRESS")
}

func (o EnvConfigProvider) Smtp_port() string {
	return os.Getenv("SMTP_PORT")
}
