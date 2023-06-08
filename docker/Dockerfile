FROM rust:1.65

WORKDIR /fbbe

COPY ./ ./

#RUN cargo build --release

RUN cargo install --path .

EXPOSE 3000

CMD ["fbbe"]
