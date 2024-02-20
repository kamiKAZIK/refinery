FROM alpine:3.19.1

COPY ./target/release/refinery /usr/local/bin/refinery
CMD ["refinery"]
