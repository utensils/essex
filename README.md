# Template

[![Build Status](https://travis-ci.org/jamesbrink/docker-template.svg?branch=master)](https://travis-ci.org/jamesbrink/docker-template) [![Docker Automated build](https://img.shields.io/docker/automated/jamesbrink/template.svg)](https://hub.docker.com/r/jamesbrink/template/) [![Docker Pulls](https://img.shields.io/docker/pulls/jamesbrink/template.svg)](https://hub.docker.com/r/jamesbrink/template/) [![Docker Stars](https://img.shields.io/docker/stars/jamesbrink/template.svg)](https://hub.docker.com/r/jamesbrink/template/) [![](https://images.microbadger.com/badges/image/jamesbrink/template.svg)](https://microbadger.com/images/jamesbrink/template "Get your own image badge on microbadger.com") [![](https://images.microbadger.com/badges/version/jamesbrink/template.svg)](https://microbadger.com/images/jamesbrink/template "Get your own version badge on microbadger.com")  


## About

This is a just a template project for my personal docker containers.

## Usage

Extending from this image. 

```Dockerfile
FROM jamesbrink/template
COPY ./MyApp /MyApp
RUN apk add --update my-deps...
```

Running a simple glxgears test. 

```shell
$ docker run -i -t --rm jamesbrink/template bash
```

## Environment Variables


| Variable                | Default Value  | Description                                                    |
| ----------------------- | -------------- | -------------------------------------------------------------- |
| `ENV`              | `DEFAULT_VALUE` | Description |

