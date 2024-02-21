FROM debian:12.5-slim

RUN groupadd -r refinery --gid=999 \
  && useradd -r -g refinery --uid=999 refinery

COPY --chown=refinery:refinery config/default.toml /etc/refinery/default.toml
COPY --chown=refinery:refinery target/release/refinery /usr/local/bin/refinery

CMD ["refinery"]
