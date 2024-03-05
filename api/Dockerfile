ARG RUST_VERSION=1.76.0
ARG APP_NAME=lamp
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
COPY . /app/
WORKDIR /app
RUN cargo build --locked --release && cp ./target/release/${APP_NAME} /bin/${APP_NAME}

FROM debian AS final
ARG APP_NAME

COPY --from=build /bin/${APP_NAME} /bin/lamp
COPY --from=build /app/configs /configs

# What the container should run when it is started.
ENTRYPOINT ["/bin/lamp"]