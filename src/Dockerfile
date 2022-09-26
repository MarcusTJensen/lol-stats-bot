FROM rust:1.31

WORKDIR /opt/app
COPY . .

RUN cargo install --path .

CMD [lol_stats_bot]