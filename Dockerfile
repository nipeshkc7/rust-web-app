# build
FROM rust:1.54 as build

COPY ./ ./
# Build your program for release
RUN cargo build --release

# production 
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build ./target/release/companies .

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

# set the startup command to run your binary
CMD ["./companies"]
