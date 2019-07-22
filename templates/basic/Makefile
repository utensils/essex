#!/usr/bin/make -f

SHELL                   := /usr/bin/env bash
DOCKER_NAMESPACE        ?= utensils
IMAGE_NAME              ?= essex-basic
BASE_IMAGE              ?= alpine:3.10
VERSION                 := $(shell git describe --tags --abbrev=0 2>/dev/null || git rev-parse --abbrev-ref HEAD 2>/dev/null)
VCS_REF                 := $(shell git rev-parse --short HEAD 2>/dev/null || echo "0000000")
BUILD_DATE              := $(shell date -u +"%Y-%m-%dT%H:%M:%SZ")

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
		--tag $(DOCKER_NAMESPACE)/$(IMAGE_NAME):latest \
		--tag $(DOCKER_NAMESPACE)/$(IMAGE_NAME):$(VCS_REF) \
		--tag $(DOCKER_NAMESPACE)/$(IMAGE_NAME):$(VERSION) \
		--file Dockerfile .

# List built images
.PHONY: list
list:
	docker images $(DOCKER_NAMESPACE)/$(IMAGE_NAME) --filter "dangling=false"

# Run any tests
.PHONY: test
test:
	docker run -t $(DOCKER_NAMESPACE)/$(IMAGE_NAME) env | grep VERSION | grep $(VERSION)

# Remove existing images
.PHONY: clean
clean:
	docker rmi $$(docker images $(DOCKER_NAMESPACE)/$(IMAGE_NAME) --format="{{.Repository}}:{{.Tag}}") --force
