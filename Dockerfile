ARG RUST_VERSION=1.76

#FROM rust:${RUST_VERSION}-slim-bookworm AS builder
#FROM rust:slim-buster AS builder
FROM rust:alpine as builder

RUN apk add musl-dev

RUN USER=root cargo new --bin web-app
WORKDIR ./web-app
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./


RUN rm ./target/release/deps/*
RUN cargo build --release
RUN strip target/release/blue-green
RUN ls -ltr target/

FROM alpine:latest
# WORKDIR /app
#RUN apk update \
#    && apk add openssl ca-certificates

ARG APP=/usr/src/app

#RUN apt-get update \
#    && apt-get install -y ca-certificates tzdata \
#    && rm -rf /var/lib/apt/lists/*
#
EXPOSE 3000

ENV TZ=Etc/UTC \
    APP_USER=api_user \
    APP_GROUP=api_group

RUN addgroup -S $APP_GROUP && adduser -S $APP_USER -G $APP_GROUP
RUN mkdir -p ${APP}

#RUN addgroup $APP_GROUP \
#    && adduser -g $APP_GROUP $APP_USER \
#    && mkdir -p ${APP}

COPY --from=builder /web-app/target/release/blue-green ${APP}/web-app/.

RUN chown -R $APP_USER:$APP_GROUP ${APP}

USER $APP_USER
WORKDIR ${APP}

RUN ls -ltr ./web-app

CMD ["./web-app/blue-green"]