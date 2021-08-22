FROM rust:1.54

COPY ./ ./

EXPOSE 8000

# Build your program for release
RUN cargo build
RUN cargo build --release

# Run the binary
CMD ["./target/release/rest"]
