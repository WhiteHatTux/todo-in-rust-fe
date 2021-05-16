FROM rust:1.52.0 as build-wasm

WORKDIR /usr/app/workdir
RUN cargo install wasm-pack
RUN rustup target add wasm32-unknown-unknown

# start cargo workaround for docker layer cache
RUN mkdir src
RUN echo "fn main() {}" > src/lib.rs
COPY Cargo.toml .
COPY Cargo.lock .
RUN echo '\n[[bin]]\nname = "download-only"\npath = "src/dummy.rs"\n' >> Cargo.toml
RUN cargo fetch
RUN wasm-pack build
RUN rm Cargo.toml src/lib.rs
# end cargo workaround for docker layer cache

COPY . .
# we need to touch this. Otherwise wasm-pack thinks that nothing needs to be compiled
RUN touch src/lib.rs
RUN wasm-pack build


FROM node:14 as build-npm
WORKDIR /app
COPY --from=build-wasm /usr/app/workdir/pkg /app/pkg
COPY --from=build-wasm /usr/app/workdir/www /app/www
RUN cd www && npm install --ci && npm run-script build



FROM nginx

COPY --from=build-npm /app/www/dist/* /usr/share/nginx/html/
COPY nginx.conf /etc/nginx/conf.d/default.conf
