ARG BASE_IMAGE=alpine:latest
FROM ${BASE_IMAGE}

# Create our group & user.
RUN addgroup -g 1000 -S template; \
    adduser -u 1000 -S -h /template -s /bin/sh -G template template

# Install deps.
ARG VERSION
RUN set -xe; \
    apk add --update  --no-cache --virtual .runtime-deps \
        ca-certificates \
        tzdata; \
    echo "${VERSION}" > /template/version.txt; \
    chown template:template /template/version.txt;

# Copy our entrypoint into the container.
COPY ./runtime-assets /

# Build arguments.
ARG VCS_REF
ARG BUILD_DATE
ARG VERSION

# Labels / Metadata.
LABEL maintainer="James Brink, brink.james@gmail.com" \
    org.label-schema.build-date="${BUILD_DATE}" \
    org.label-schema.decription="Template" \
    org.label-schema.name="template" \
    org.label-schema.vendor="Utensils" \
    org.label-schema.schema-version="1.0.0-rc1" \
    org.label-schema.vcs-ref="${VCS_REF}" \
    org.label-schema.vcs-url="https://github.com/utensils/docker-template" \
    org.label-schema.version="${VERSION}"

# Setup our environment variables.
ENV PATH="/usr/local/bin:$PATH"

# Drop down to our unprivileged user.
USER template

# Set our working directory.
WORKDIR /template

# Set the entrypoint.
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]

# Set the default command
CMD ["/bin/sh"]
