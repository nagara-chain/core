FROM ubuntu:22.04

LABEL org.opencontainers.image.authors "nagara Developers <dev@nagara.network>"
LABEL org.opencontainers.image.source "https://github.com/nagara-network/core"
LABEL org.opencontainers.image.description "nagara Core Network"

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libc6 \
    libstdc++6 \
    libzstd1 && \
    apt-get autoremove -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY target/release/nagara-core-node nagara-core-node
RUN ln -s /app/nagara-core-node /usr/local/bin/nagara-core-node

ENTRYPOINT [ "nagara-core-node" ]
CMD [ "--help" ]
