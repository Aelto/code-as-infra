FROM docker.io/golang:1.26-alpine AS build

WORKDIR /app
COPY go.mod ./
RUN go mod download
COPY **.go ./

RUN CGO_ENABLED=0 GOOS=linux go build -o /mailqueue

FROM gcr.io/distroless/static-debian13 as release

COPY --from=build /mailqueue /mailqueue

EXPOSE 3000
CMD [ "/mailqueue" ]
