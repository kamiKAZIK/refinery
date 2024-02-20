FROM debian:12.5-slim

COPY ./target/release/refinery /usr/local/bin/refinery
CMD ["refinery"]
