# Basic

 [![Docker Automated build](https://img.shields.io/docker/automated/utensils/template.svg)](https://hub.docker.com/r/utensils/template/) [![Docker Pulls](https://img.shields.io/docker/pulls/utensils/template.svg)](https://hub.docker.com/r/utensils/template/) [![Docker Stars](https://img.shields.io/docker/stars/utensils/template.svg)](https://hub.docker.com/r/utensils/template/) [![](https://images.microbadger.com/badges/image/utensils/template.svg)](https://microbadger.com/images/utensils/template "Get your own image badge on microbadger.com") [![](https://images.microbadger.com/badges/version/utensils/template.svg)](https://microbadger.com/images/utensils/template "Get your own version badge on microbadger.com")  


## About

This is basic boilerplate for a docker based project. 

## Building

To build the project:
```shell
make
```

To list the images:
```shell
make list
```

To run any tests:
```shell
make test
```

To push image to remote docker repository:
```shell
REPO_PASSWORD='MyPassword!$' make push
```

To update README on remote docker repository (docker hub):

```shell
REPO_PASSWORD='MyPassword!$' make push-readme
```

To cleanup and remove built images:
```shell
make clean
```

## Usage

To run the container:
```shell
docker run -i -t utensils/template
```


## Environment Variables


| Variable | Default Value   | Description |
| -------- | --------------- | ----------- |
| `ENV`    | `DEFAULT_VALUE` | Description |

