# syntax=docker/dockerfile:1
FROM rust:1.72-slim-bullseye AS builder
LABEL authors="gunwoo"

WORKDIR /frontend
COPY ./frontend /frontend

WORKDIR /backend
COPY ./backend /backend

RUN cargo build --release

EXPOSE 4000
ENTRYPOINT ["./frontend"]

CMD ["trunk","serve"]