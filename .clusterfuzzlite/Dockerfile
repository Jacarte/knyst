FROM gcr.io/oss-fuzz-base/base-builder-rust

RUN apt-get update && apt-get install -y make autoconf automake libtool curl \
  cmake python llvm-dev libclang-dev clang \
  libgmp-dev python3-pip pkg-config libasound2-dev libasound2:i386 libasound2

RUN pip install google-cloud-ndb==1.7.0
# FIX for https://github.com/rust-lang/rust/issues/98833
# Freshly install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

RUN rustup update nightly
RUN rustup update stable

COPY . $SRC/knyst
WORKDIR $SRC/knyst

COPY ./.clusterfuzzlite/build.sh $SRC/
