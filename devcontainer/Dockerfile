#Base Container 
FROM rust:latest
#Add the cargo to the PATH
RUN echo "export PATH=$PATH:/usr/local/cargo/bin" >> /root/.bashrc
#Install the rust tools we want
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk cargo-watch
