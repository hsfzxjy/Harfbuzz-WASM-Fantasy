FROM hsfzxjy/proxychains-ng:debian-bullseye-v4.15 as pcng

FROM ubuntu:22.04 as base
RUN sed -i s/archive.ubuntu.com/mirrors.ustc.edu.cn/g /etc/apt/sources.list
ENV DEBIAN_FRONTEND noninteractive

FROM base as dev-base
ARG USE_PROXY
COPY --from=pcng /opt/pcng /opt/pcng
ENV PATH=/opt/pcng:/root/.cargo/bin:$PATH
RUN apt-get update && apt-get install -yq git

FROM dev-base as dev
ARG USE_PROXY
RUN apt-get install -yq \
    wget \
    curl \
    python3 \
    python3-pip \
    cmake \
    gcc \
    ninja-build libglib2.0 libxft2 libxft-dev
RUN pcng python3 -m pip install fontmake
RUN pcng curl -sSf https://sh.rustup.rs > /rustup-install && \
    pcng sh /rustup-install -y && \
    pcng cargo install wasm-pack
RUN apt-get install -yq openssl libssl-dev
RUN cd /tmp && pcng cargo install cargo-generate
RUN export USER=root && export HOME=/root && \
    pcng cargo generate --git https://github.com/rustwasm/wasm-pack-template --destination /tmp --name tmp && \
    cd /tmp/tmp && pcng wasm-pack build
RUN pcng apt-get install meson pkg-config ragel gtk-doc-tools gcc g++ libfreetype6-dev libglib2.0-dev libcairo2-dev -yq
RUN pcng apt-get install -yq ccache
COPY --from=iwasm requirements.txt /tmp
RUN pcng pip install -r /tmp/requirements.txt

FROM base as final
ARG USE_PROXY
ENV LC_ALL C.UTF-8
ENV LANG en_US.UTF-8
ENV LANGUAGE en_US.UTF-8
RUN apt-get update
RUN apt-get install gedit libglib2.0-bin dbus-x11 -yq
RUN mkdir /fantasy
COPY --from=iwasm  libiwasm.so /fantasy/
COPY --from=hb src/libharfbuzz.so.0.60811.0 /fantasy/
ENV LD_PRELOAD="/fantasy/libharfbuzz.so.0.60811.0 /fantasy/libiwasm.so"
COPY ./scripts/start-gedit.sh /fantasy
COPY --from=font HB_WASM_Fantasy.ttf /usr/share/fonts/
COPY --from=wamr wamrc /fantasy
CMD [ "/fantasy/start-gedit.sh" ]