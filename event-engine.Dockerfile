# must be underscore otherwise RUN rm ./target/release/deps/${my_appname}* will fail
ARG my_appname=event_engine

FROM rust:bookworm as build
ARG my_appname
ENV my_appname=$my_appname

WORKDIR /workspace/

# copy over workspace manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# copy local dependencies
COPY ./rust-crates/types/ ./rust-crates/types/
RUN touch ./rust-crates/types/src/lib.rs

# copy the crate we're building
COPY ./rust-crates/event-engine/ ./rust-crates/event-engine/
RUN touch ./rust-crates/event-engine/src/main.rs

# build, with dependency cache
RUN --mount=type=cache,target=/usr/local/cargo/registry,id="reg-${TARGETPLATFORM}" \
    --mount=type=cache,target=target,id="target-${my_appname}-${TARGETPLATFORM}" \
    cargo build --release && \
    mkdir bin && \
    mv target/release/${my_appname} bin/${my_appname}


# our final base
FROM debian:bookworm-slim
ARG my_appname
ENV my_appname=$my_appname

# install OS dependencies
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

# copy the build artifact from the build cache
COPY --from=build /workspace/bin/${my_appname} /usr/local/bin/${my_appname}

# set the startup command to run your binary
CMD ${my_appname}
