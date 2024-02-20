FROM alpine:3

COPY ./target/release/refinery /usr/local/bin/refinery
CMD ["refinery"]
