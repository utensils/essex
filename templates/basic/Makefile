#!/usr/bin/make -f

SHELL                 := /usr/bin/env bash
REPO_NAMESPACE        ?= {{ repo_namespace }}
REPO_USERNAME         ?= {{ repo_username }}
REPO_API_URL          ?= https://hub.docker.com/v2
IMAGE_NAME            ?= {{ image_name }}
BASE_IMAGE            ?= alpine:3.21
SED                   := $(shell [[ `command -v gsed` ]] && echo gsed || echo sed)
VERSION               := $(shell git describe --tags --abbrev=0 2>/dev/null || git rev-parse --abbrev-ref HEAD | $(SED) 's|/|_|g' 2>/dev/null)
VCS_REF               := $(shell git rev-parse --short HEAD 2>/dev/null || echo "0000000")
BUILD_DATE            := $(shell date -u +"%Y-%m-%dT%H:%M:%SZ")

# Default target is to build container
.PHONY: default
default: build

# Build the docker image
.PHONY: build
build: list
	docker build \
		--build-arg BASE_IMAGE=$(BASE_IMAGE) \
		--build-arg BUILD_DATE=$(BUILD_DATE) \
		--build-arg VCS_REF=$(VCS_REF) \
		--build-arg VERSION=$(VERSION) \
		--tag $(REPO_NAMESPACE)/$(IMAGE_NAME):latest \
		--tag $(REPO_NAMESPACE)/$(IMAGE_NAME):$(VCS_REF) \
		--tag $(REPO_NAMESPACE)/$(IMAGE_NAME):$(VERSION) \
		--file Dockerfile .

# List built images
.PHONY: list
list:
	docker images $(REPO_NAMESPACE)/$(IMAGE_NAME) --filter "dangling=false"

# Run any tests
.PHONY: test
test:
	docker run -t $(REPO_NAMESPACE)/$(IMAGE_NAME) env | grep VERSION | grep $(VERSION)

# Push images to repo
.PHONY: push
push:
	echo "$$REPO_PASSWORD" | docker login -u "$(REPO_USERNAME)" --password-stdin; \
		docker push  $(REPO_NAMESPACE)/$(IMAGE_NAME):latest; \
		docker push  $(REPO_NAMESPACE)/$(IMAGE_NAME):$(VCS_REF); \
		docker push  $(REPO_NAMESPACE)/$(IMAGE_NAME):$(VERSION);

# Remove existing images
.PHONY: clean
clean:
	docker rmi $$(docker images $(REPO_NAMESPACE)/$(IMAGE_NAME) --format="{%raw%}{{.Repository}}:{{.Tag}}{%endraw%}") --force