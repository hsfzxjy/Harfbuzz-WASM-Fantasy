FROM hsfzxjy/proxychains-ng:debian-bullseye-v4.15 as pcng

FROM ubuntu:20.04 as base
RUN sed -i s/archive.ubuntu.com/mirrors.ustc.edu.cn/g /etc/apt/sources.list
ENV DEBIAN_FRONTEND noninteractive

FROM base as dev-base
ARG USE_PROXY=
COPY --from=pcng /opt/pcng /opt/pcng
ENV PATH=/opt/pcng:/root/.cargo/bin:$PATH
RUN apt-get update && apt-get install -yq git

FROM dev-base as dev
RUN apt-get install -yq \
    wget \
    curl \
    python3 \
    python3-pip \
    cmake \
    gcc \
    ninja-build libglib2.0 libxft2 libxft-dev
RUN python3 -m pip install fontmake
RUN curl -sSf https://sh.rustup.rs > /rustup-install && \
    sh /rustup-install -y && \
    cargo install wasm-pack

FROM dev-base as source
RUN pcng git clone https://github.com/hsfzxjy/Harfbuzz-WASM-Fantasy /repo && \
    cd /repo && \
    pcng git submodule update --init --recursive

FROM dev as build-base
COPY --from=source /repo /repo
WORKDIR /repo

FROM build-base as build-hb
RUN pcng make harfbuzz

FROM build-base as build-font
RUN pcng make

FROM build-font as build-font-local
COPY ./scripts ./src ./Cargo.toml ./Makefile /repo/
RUN pcng make

FROM base as final
RUN apt-get update
ENV LC_ALL C.UTF-8
ENV LANG en_US.UTF-8
ENV LANGUAGE en_US.UTF-8
RUN apt-get install gedit -yq
RUN mkdir /fantasy
COPY --from=build-font-local /repo/bin/HB_WASM_Fantasy.ttf /usr/share/fonts/
COPY --from=build-hb /repo/harfbuzz/build/src/libharfbuzz.so.0.60811.0 /repo/wasm-micro-runtime/build/libiwasm.so  /fantasy/
COPY ./scripts/start-gedit.sh /fantasy
ENV LD_PRELOAD="/fantasy/libharfbuzz.so.0.60811.0 /fantasy/libiwasm.so"
CMD [ "/fantasy/start-gedit.sh" ]
