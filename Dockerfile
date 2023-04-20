FROM rust:1.68

COPY . .

RUN cargo build --release

EXPOSE 8080 8080

CMD [ "cargo", "run", "--release" ]