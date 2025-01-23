# select build image
FROM wfm-be as build

# copy over your manifests
# COPY ./Cargo.lock ./Cargo.lock
# COPY ./Cargo.toml ./Cargo.toml

# copy your source tree
COPY . /app

# build for release
RUN cargo build --release

# our final base
# Stage 2: Final image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*


# copy the build artifact from the build stage
COPY --from=build /app/target/release/wfm-be .

# set the startup command to run your binary
CMD ["./wfm-be"]