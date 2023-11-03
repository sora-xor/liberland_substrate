FROM debian:bookworm-20231030-slim
WORKDIR /app
# Install dependencies
RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    ca-certificates nano curl && \
    apt-get autoremove -y && \
    apt-get clean && \
    find /var/lib/apt/lists/ -type f -not -name lock -delete
COPY specs /specs
COPY target /target
RUN useradd substrate -u 10000
RUN mkdir /chain && \
    chown 10000:10000 /chain /specs
USER substrate

COPY ./target/release/substrate /usr/local/bin/substrate

ENTRYPOINT ["substrate"]