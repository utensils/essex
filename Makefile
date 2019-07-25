#!/usr/bin/make -f

SHELL                   := /usr/bin/env bash
REPO_NAMESPACE        ?= utensils
TEMPLATES               := basic

# Build the docker image
.PHONY: $(TEMPLATES)
$(TEMPLATES):
	make_goals="$(MAKECMDGOALS)"; \
	goals="$${make_goals#$(@)}"; \
	$(MAKE) -C templates/$@ $$goals;
	
# Kind of hacky, catch non-existent targets here?
%:
	
