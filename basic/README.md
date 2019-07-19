# Template

[![Build Status](https://travis-ci.org/utensils/docker-template.svg?branch=master)](https://travis-ci.org/utensils/docker-template) [![Docker Automated build](https://img.shields.io/docker/automated/utensils/template.svg)](https://hub.docker.com/r/utensils/template/) [![Docker Pulls](https://img.shields.io/docker/pulls/utensils/template.svg)](https://hub.docker.com/r/utensils/template/) [![Docker Stars](https://img.shields.io/docker/stars/utensils/template.svg)](https://hub.docker.com/r/utensils/template/) [![](https://images.microbadger.com/badges/image/utensils/template.svg)](https://microbadger.com/images/utensils/template "Get your own image badge on microbadger.com") [![](https://images.microbadger.com/badges/version/utensils/template.svg)](https://microbadger.com/images/utensils/template "Get your own version badge on microbadger.com")  


## About

This is a just a template.skeleton project for docker containers.

## Usage

Extending from this image. 

```Dockerfile
FROM utensils/template
COPY ./MyApp /MyApp
RUN apk add --update my-deps...
```


## Environment Variables


| Variable | Default Value   | Description |
| -------- | --------------- | ----------- |
| `ENV`    | `DEFAULT_VALUE` | Description |

