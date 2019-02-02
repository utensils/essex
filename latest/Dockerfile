FROM alpine:3.9

# Build arguments.
ARG VCS_REF
ARG BUILD_DATE
ARG TEMPLATE_VERSION="0.1.0"

# Labels / Metadata.
LABEL maintainer="James Brink, brink.james@gmail.com" \
    decription="Template" \
    version="${TEMPLATE_VERSION}" \
    org.label-schema.name="template" \
    org.label-schema.build-date="${BUILD_DATE}" \
    org.label-schema.vcs-ref="${VCS_REF}" \
    org.label-schema.vcs-url="https://github.com/jamesbrink/docker-template" \
    org.label-schema.schema-version="1.0.0-rc1"

# Create our group & user.
RUN addgroup -g 1000 -S template \
    && adduser -u 1000 -S -h /template -s /bin/sh -G template template

# Install deps.
RUN set -xe; \
    apk add --update  --no-cache --virtual .runtime-deps bash; \
    echo "${TEMPLATE_VERSION}" > /template/version.txt; \
    chown template:template /template/version.txt;

# Copy our entrypoint into the container.
COPY ./docker-assets /

# Setup our environment variables.
ENV PATH="/usr/local/bin:$PATH"

# Drop down to our unprivileged user.
USER template

# Set our working directory.
WORKDIR /template

# Set the entrypoint.
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]

# Set the default command
CMD ["/bin/bash"]
