# syntax=docker/dockerfile:1
FROM rust:1.72-slim-bullseye AS builder
LABEL authors="gunwoo"

WORKDIR /frontend
COPY . .

EXPOSE 4000

CMD ["trunk","serve"]