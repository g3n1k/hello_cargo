# select build image
FROM rust:1.84 as build

# create a new empty shell project
RUN USER=root cargo new --bin my_project
WORKDIR /my_project

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/my_project*
RUN cargo build --release

# our final base
# Stage 2: Final image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*


# copy the build artifact from the build stage
COPY --from=build /my_project/target/release/my_project .

# set the startup command to run your binary
CMD ["./my_project"]