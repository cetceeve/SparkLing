# must be underscore otherwise RUN rm ./target/release/deps/${my_appname}* will fail
ARG my_appname=rt_data_exporter

FROM rust:bookworm as build
ARG my_appname
ENV my_appname=$my_appname

# create a new empty shell project
RUN USER=root cargo new --bin ${my_appname}
WORKDIR /${my_appname}

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY . .

# build for release
RUN rm ./target/release/deps/${my_appname}*
RUN cargo build --release

# our final base
FROM debian:bookworm-slim
ARG my_appname
ENV my_appname=$my_appname

# install OS dependencies
# RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

# copy the build artifact from the build stage
COPY --from=build /${my_appname}/target/release/${my_appname} /usr/local/bin/${my_appname}

# set the startup command to run your binary
CMD ${my_appname}