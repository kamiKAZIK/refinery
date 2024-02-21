FROM debian:12.5-slim

COPY config/default.toml /etc/refinery/default.toml
COPY target/release/refinery /usr/local/bin/refinery

CMD ["refinery"]
