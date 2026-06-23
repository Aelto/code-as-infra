FROM docker.io/golang:1.26-alpine

WORKDIR /app
COPY go.mod ./
RUN go mod download
COPY **.go ./

RUN CGO_ENABLED=0 GOOS=linux go build -o /mailqueue

EXPOSE 3000

CMD [ "/mailqueue" ]
