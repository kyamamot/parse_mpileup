FROM rust:1.66.0-buster
MAINTAINER Keisuke Yamamoto

WORKDIR /tools

RUN git clone https://github.com/kyamamot/parse_mpileup.git && \
    cd parse_mpileup && \
    cargo build --release

CMD ["/bin/bash"]
