# Docker image for running Frequency parachain node container (with collating)
# locally in instant seal mode. Requires to run from repository root and to copy
# the binary in the build folder.
# This is the build stage for Polkadot. Here we create the binary in a temporary image.
FROM --platform=linux/amd64 ubuntu:20.04 AS base

LABEL maintainer="Frequency"
LABEL description="Frequency collator node in instant seal mode"

RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates

RUN apt-get -y install git

RUN git clone https://github.com/LibertyDSNP/schemas.git

# This is the 2nd stage: a very small image where we copy the Frequency binary
FROM --platform=linux/amd64 node:18.12.1

RUN groupmod -g 1001 node \
  && usermod -u 1001 -g 1001 node

RUN useradd -m -u 1000 -U -s /bin/sh -d /frequency frequency && \
	mkdir -p /data /frequency/.local/share && \
	chown -R frequency:frequency /data && \
	ln -s /data /frequency/.local/share/frequency

RUN npm --version

USER frequency

RUN npm --version

COPY --from=base /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
# For local testing only
#COPY --chown=frequency target/release/frequency.amd64 ./frequency/frequency
COPY --chown=frequency target/release/frequency ./frequency/
RUN chmod +x ./frequency/frequency

COPY --chown=frequency scripts/deploy_schemas.sh ./frequency/
RUN chmod +x ./frequency/deploy_schemas.sh

COPY --from=base --chown=frequency /schemas/ /frequency/schemas
#RUN chmod +x ./frequency/schemas

# 9933 P2P port
# 9944 for RPC call
# 30333 for Websocket
EXPOSE 9933 9944 30333

VOLUME ["/data"]

##TODO: figure out why this errors out do to not existing
#ENTRYPOINT ["/usr/bin/tini", "--"]

# Params which can be overriden from CLI
CMD ["/bin/bash", "/frequency/deploy_schemas.sh"]
