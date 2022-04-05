FROM ghcr.io/plc-lang/rust-llvm:latest-linux

#Add the arm64 arch 
RUN dpkg --add-architecture arm64 \
	&& apt-get update
RUN apt-get install -y \
	gcc-aarch64-linux-gnu g++-aarch64-linux-gnu \
	musl musl-dev musl:arm64 musl-dev:arm64

#Install x86_64-musl and aarch-musl targets
RUN rustup target add aarch64-unknown-linux-musl \
	&& rustup target add x86_64-unknown-linux-musl \
	&& rustup target add aarch64-unknown-linux-gnu

