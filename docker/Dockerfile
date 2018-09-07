FROM ubuntu:18.04

ENV RUSTUP_HOME /opt/rustup
ENV CARGO_HOME /opt/cargo

RUN apt-get update && \
    apt-get install -y \
      curl bash build-essential

RUN curl https://sh.rustup.rs -sSf -o /tmp/install-rust.sh && \
    chmod u+x /tmp/install-rust.sh && \
    /tmp/install-rust.sh --no-modify-path -v -y

RUN mkdir ${CARGO_HOME}/registry && \
    chmod a+w -R ${CARGO_HOME}/registry

RUN echo 'PATH=$PATH:/opt/cargo/bin' > /etc/profile.d/rust.sh
RUN echo 'cd ~' >> /etc/profile.d/rust.sh

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/bin/sh", "/entrypoint.sh"]
