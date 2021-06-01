FROM rust:1.52.1-alpine3.13 AS buildStage

RUN apk add --no-cache libc-dev openssl-dev

COPY . /src
WORKDIR /src

ENV CARGO_INSTALL_ROOT /
ENV RUSTFLAGS -Ctarget-feature=-crt-static

RUN cargo install --path .

FROM alpine:3.13 

RUN apk add --no-cache openssl libgcc
COPY --from=buildStage /bin /bin
COPY --from=buildStage /src/assets /bin/assets
WORKDIR /bin

CMD ["./baebot"]
