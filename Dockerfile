# ===== BUILD ======

FROM phusion/baseimage:0.10.2 as builder
LABEL maintainer="vlbos2018@gmail.com"


ARG PROFILE=release

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang build-essential clang libclang-dev curl

# Get project and run it
#RUN git clone https://github.com/vlbos/pacific-store-node /pacific_store_node
RUN mkdir pacific_store_node
WORKDIR /pacific_store_node
COPY . .

# RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
RUN  curl https://sh.rustup.rs -sSf | sed "s/https:\/\/static.rust-lang.org\/rustup\/dist/https:\/\/mirrors.ustc.edu.cn\/rust-static\/rustup\/dist/g" | sh -s -- -y && \
echo "[source.crates-io]" >> $HOME/.cargo/config  && \
echo "registry = \"https://github.com/rust-lang/crates.io-index\" " >> $HOME/.cargo/config  && \
echo "replace-with = 'ustc' " >> $HOME/.cargo/config  && \
echo "[source.ustc] " >> $HOME/.cargo/config  && \
echo "registry = \"git://mirrors.ustc.edu.cn/crates.io-index\" " >> $HOME/.cargo/config  && \
export PATH="$PATH:$HOME/.cargo/bin" && \
	# rustup toolchain uninstall $(rustup toolchain list) && \
    rustup toolchain install nightly && \
	rustup target add wasm32-unknown-unknown --toolchain nightly && \
	rustup default nightly && \
	rustup default stable 
    # rustup target list --installed && \
    # rustup show && \
RUN	export PATH="$PATH:$HOME/.cargo/bin" && \
 export CARGO_HTTP_MULTIPLEXING=false && \
    cargo build "--$PROFILE" 
	# && \
	# cargo test

RUN cd target/release && ls -la

# ===== RUN ======

FROM phusion/baseimage:0.10.2
ARG PROFILE=release

COPY --from=builder /pacific_store_node/target/$PROFILE/pacific_store /usr/local/bin

EXPOSE 9944
VOLUME ["/node-data"]

# Copy and run start script
COPY ["./run.sh", "./run.sh"]
RUN chmod +x ./run.sh
CMD ["bash", "-c", "./run.sh"]
