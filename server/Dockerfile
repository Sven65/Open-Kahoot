FROM --platform="linux/amd64" debian:bullseye-20240211-slim as build

COPY ./install-packages.sh .
RUN chmod +x ./install-packages.sh 
RUN ./install-packages.sh

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN USER=root cargo new --bin open-kahoot
WORKDIR /open-kahoot

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build deps for cache

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/app/target \
    cargo build --release
RUN rm src/*.rs

COPY ./src ./src

# Build for release

RUN cargo install --path .

RUN ls /open-kahoot/target/release

# Final build

FROM --platform="linux/amd64" debian:bullseye-20240211-slim

COPY ./install-packages.sh .
RUN chmod +x ./install-packages.sh 
RUN ./install-packages.sh

RUN chmod a+x /etc/ssl/certs

RUN update-ca-certificates

WORKDIR /home

COPY --from=build /open-kahoot/target/release/ok-server /home/ok-server


EXPOSE 8000
CMD ["/home/ok-server"]