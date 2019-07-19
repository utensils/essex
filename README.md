# Essex - Docker boilerplate

## About

The goal of this project is to provide a consistent boilerplate and templates for creating docker projects, or integrating docker into existing projects. It will provide the essential skelton with proper labeling and metadata out of the box and will be powered using a `Makefile` workflow with targets for the essential tasks.

## Goals

 * All images will have valid Labels/Annotations. See the Open Container Initiative [image-spec](https://github.com/opencontainers/image-spec/blob/master/annotations.md)


## Installation

To install Essex you can either clone this repository to any location on your machine and add it to your `$PATH`.
Or you can install with the following:  
```shell
\curl -sSL  https://raw.githubusercontent.com/utensils/essex/master/install.sh | bash -s
```