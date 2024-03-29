FROM debian:12.5-slim

RUN groupadd -r refinery --gid=999 \
  && useradd -r -g refinery --uid=999 refinery

COPY --chown=refinery:refinery config/default.toml /etc/refinery/default.toml
COPY --chown=refinery:refinery config/log4rs.yaml /etc/refinery/log4rs.yaml
COPY --chown=refinery:refinery entrypoint.sh /usr/local/bin/entrypoint.sh
COPY --chown=refinery:refinery target/release/refinery /usr/local/bin/refinery

ENTRYPOINT ["entrypoint.sh"]
CMD ["refinery"]
