ARG BASE_IMAGE=alpine:latest
FROM ${BASE_IMAGE}

# Create our group & user.
RUN addgroup -g 1000 -S essex; \
    adduser -u 1000 -S -h /essex -s /bin/sh -G essex essex

# Install deps.
ARG VERSION
RUN set -xe; \
    apk add --update  --no-cache --virtual .runtime-deps \
        ca-certificates \
        tzdata; \
    echo "${VERSION}" > /essex/version.txt; \
    chown essex:essex /essex/version.txt;

# Copy our entrypoint into the container.
COPY ./runtime-assets /

# Build arguments.
ARG VCS_REF
ARG BUILD_DATE
ARG VERSION

# Labels / Metadata.
LABEL \
    org.opencontainers.image.authors="James Brink <brink.james@gmail.com>" \
    org.opencontainers.image.created="${BUILD_DATE}" \
    org.opencontainers.image.description="Essex basic image example" \
    org.opencontainers.image.revision="${VCS_REF}" \
    org.opencontainers.image.source="https://github.com/utensils/essex" \
    org.opencontainers.image.title="essex-basic" \
    org.opencontainers.image.vendor="Utensils" \
    org.opencontainers.image.version="${VERSION}" \
    org.opencontainers.image.version="1.0.0-rc1"

# Setup our environment variables.
ENV PATH="/usr/local/bin:$PATH"

# Drop down to our unprivileged user.
USER essex

# Set our working directory.
WORKDIR /essex

# Set the entrypoint.
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]

# Set the default command
CMD ["/bin/sh"]
