# Pelican yolk for Horizon's dedicated server (the engine needs glibc 2.43: Ubuntu 26.04).
FROM ubuntu:26.04
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates tini iproute2 \
 && rm -rf /var/lib/apt/lists/* \
 && useradd -m -d /home/container -s /bin/bash container
USER container
ENV USER=container HOME=/home/container
WORKDIR /home/container
STOPSIGNAL SIGINT
COPY entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/usr/bin/tini", "-g", "--"]
CMD ["/entrypoint.sh"]
