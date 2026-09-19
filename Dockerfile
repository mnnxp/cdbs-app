# select build image
FROM rust:1.84.1 AS build

# Install wasm target
RUN rustup target add wasm32-unknown-unknown

# prepare trunk v0.21.14 and wasm-bindgen-0.2.92 (reserve way)
# RUN cargo install --version 0.21.14 trunk --locked && \
#     cargo install --version 0.2.92 wasm-bindgen-cli --locked

# Install Trunk (binary)
RUN curl -fsSL https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz \
    | tar -xz -C /usr/local/bin/

# Install wasm-bindgen-cli (binary)
RUN curl -fsSL https://github.com/rustwasm/wasm-bindgen/releases/download/0.2.92/wasm-bindgen-0.2.92-x86_64-unknown-linux-musl.tar.gz \
    | tar -xz -C /usr/local/bin/ --strip-components=1

WORKDIR /cdbs-app

# Copy source tree
COPY ./assets/ ./assets/
COPY ./graphql/ ./graphql/
COPY ./src/ ./src/
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./index.html ./index.html
COPY ./trunk.toml ./trunk.toml

# Build for release with Clipboard API flag
RUN RUSTFLAGS="--cfg=web_sys_unstable_apis" trunk --config /cdbs-app/trunk.toml build

RUN echo "===WASM COMPILED==="
RUN ls -lh ./dist

# final base
FROM httpd:2.4-alpine

RUN apk update && apk upgrade --no-cache

# Enable SSL and required modules
RUN sed -i \
    -e 's/^#\(Include conf\/extra\/httpd-ssl.conf\)/\1/' \
    -e 's/^#\(LoadModule ssl_module modules\/mod_ssl.so\)/\1/' \
    -e 's/^#\(LoadModule socache_shmcb_module modules\/mod_socache_shmcb.so\)/\1/' \
    conf/httpd.conf

RUN sed -i \
    -e 's|/usr/local/apache2/conf/server.crt|/usr/local/apache2/conf/ssl/server.crt|g' \
    -e 's|/usr/local/apache2/conf/server.key|/usr/local/apache2/conf/ssl/server.key|g' \
    conf/extra/httpd-ssl.conf

EXPOSE 443

# copy the build artifact from the build stage
COPY --from=build /cdbs-app/dist/ /usr/local/apache2/htdocs/
