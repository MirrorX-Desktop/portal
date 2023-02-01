FROM rust:1.67.0-slim-bullseye as builder

WORKDIR /app/portal

COPY . .

RUN apt-get update && apt-get install -y --no-install-recommends curl make libssl-dev pkg-config libudev-dev && \
    curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y --no-install-recommends nodejs && \
    cd dashboard && \
    npm install && \
    npm run build && \
    cd /app/portal && \
    cargo install --path . 


FROM debian:bullseye-slim

ARG USERNAME=mirrorx
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid $USER_GID $USERNAME && \
    useradd --uid $USER_UID --gid $USER_GID -m $USERNAME && \
    mkdir -p /app/portal && \
    chown $USER_UID:$USER_GID /app/portal

WORKDIR /app/portal

USER $USERNAME

COPY --from=builder /usr/local/cargo/bin/portal .
COPY .env .

CMD ["./portal"]
