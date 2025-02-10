FROM rust:latest

WORKDIR /usr/src/app

COPY . .

RUN apt-get update && apt-get install -y libssl-dev pkg-config
RUN cargo build --release

CMD ["cargo", "run"]
