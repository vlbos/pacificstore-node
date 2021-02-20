# ===== BUILD ======

FROM phusion/baseimage:0.10.2 as builder
LABEL maintainer="vlbos2018@gmail.com"


ARG PROFILE=release

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang build-essential clang libclang-dev curl

# Get project and run it
#RUN git clone https://github.com/vlbos/pacific-store-node /pacific_store
RUN mkdir pacific_store
WORKDIR /pacific_store
COPY . .
rustup target add wasm32-unknown-unknown --toolchain nightly
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
	export PATH="$PATH:$HOME/.cargo/bin" && \
	rustup toolchain uninstall $(rustup toolchain list) && \
	rustup default stable && \
	rustup target add wasm32-unknown-unknown --toolchain nightly && \
    rustup target list --installed && \
    rustup show && \
	cargo build "--$PROFILE" 
	# && \
	# cargo test

RUN cd target/release && ls -la

# ===== RUN ======

FROM phusion/baseimage:0.10.2
ARG PROFILE=release

COPY --from=builder /pacific_store/target/$PROFILE/nft /usr/local/bin

EXPOSE 9944
VOLUME ["/chain-data"]

# Copy and run start script
COPY ["./run.sh", "./run.sh"]
RUN chmod +x ./run.sh
CMD ["bash", "-c", "./run.sh"]
