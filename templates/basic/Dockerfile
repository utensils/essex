ARG BASE_IMAGE=alpine:3.21
FROM ${BASE_IMAGE}

# Install deps.
RUN set -xe; \
    apk add --update  --no-cache --virtual .runtime-deps \
        ca-certificates \
        tzdata;

# Create our group & user.
RUN set -xe; \
    addgroup -g 1000 -S essex; \
    adduser -u 1000 -S -h /essex -s /bin/sh -G essex essex;

# Copy our entrypoint into the container.
COPY ./runtime-assets /

# Build arguments.
ARG VCS_REF
ARG BUILD_DATE
ARG VERSION

# Labels / Metadata.
LABEL \
    org.opencontainers.image.authors="{{ repo_username }} <contact@example.com>" \
    org.opencontainers.image.created="{{ build_date }}" \
    org.opencontainers.image.description="{{ image_name }} description" \
    org.opencontainers.image.revision="{{ vcs_ref }}" \
    org.opencontainers.image.source="https://github.com/{{ repo_namespace }}/{{ image_name }}" \
    org.opencontainers.image.title="{{ image_name }}" \
    org.opencontainers.image.vendor="{{ vendor }}" \
    org.opencontainers.image.version="{{ version }}"

# Setup our environment variables.
ENV \
    PATH="/usr/local/bin:$PATH" \
    VERSION="{{ version }}"

# Drop down to our unprivileged user.
USER essex

# Set our working directory.
WORKDIR /essex

# Set the entrypoint.
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]

# Set the default command
CMD ["/bin/sh"]