#################################################################################
## Builder
#################################################################################
FROM rust:1.72-bookworm AS builder
ARG USE_TLS=false
WORKDIR /app
COPY ./ .
RUN if [ ${USE_TLS} = true ] ; then cargo build --features tls --release ; else cargo build --release ; fi

#################################################################################
## Final image
#################################################################################
FROM debian:bookworm

RUN apt update && apt install -y --no-install-recommends ca-certificates libssl-dev

# Create unprivileged user
ENV USER=unprivileged
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# Set the working directory
WORKDIR /app

# Copy our build
COPY --from=builder /app/target/release/${PROJECT_NAME} /app/${PROJECT_NAME}

# Use an unprivileged user
USER ${USER}:${USER}

CMD ["sh", "-c", "/app/${PROJECT_NAME}"]
EXPOSE ${PORT}
