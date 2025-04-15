FROM rustlang/rust:nightly

# RUN apt-get update && apt-get upgrade -y \
#   && apt-get install -y libssl-dev fd-find lld mold build-essential pkg-config \
#   && rm -rf /var/lib/apt/lists/*
ENV RUST_BACKTRACE=1
# RUN apt-get update && apt-get install -y  \
#   libprotobuf-dev \
#   libssl-dev \
#   libxcb-render0-dev \
#   libxcb-shape0-dev \
#   build-essential \
#   g++ \
#   libxcb-xfixes0-dev \
#   libxcb1-dev \
#   protobuf-compiler \
# #   gobjc++-mingw-w64 \
#   fd-find \
#   # Faster builds
#   lld \
#   && rm -rf /var/lib/apt/lists/*

  
  
COPY . /app

WORKDIR /app

RUN curl -L --proto '=https' --tlsv1.2 -sSf \
  https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
  | bash
# RUN rustup show
# RUN rustup update
RUN cargo run --package tools --bin init -- setup
# RUN cargo install --git https://github.com/astral-sh/uv uv



RUN just docmdgen


RUN apt-get update && apt-get upgrade -y \
  && apt-get install -y libssl-dev fd-find lld mold build-essential pkg-config pipx --no-install-recommends \
  && rm -rf /var/lib/apt/lists/*
RUN pipx install uv
ENV RUSTFLAGS="-C link-arg=-fuse-ld=lld"

RUN cargo build