# Essex - Boilerplate for Docker Based Projects

## About

**Project is still under active development and may not work**

Essex is a CLI utility written in bash to quickly setup consistent and clean Docker projects.

## Goals

 * Allow easy creation of Dockerfiles using best practices
 * Use a `Makefile` driven build workflow
 * Easily maintain consistency between projects using that use Docker
 * All images will have valid Labels/Annotations. See the Open Container Initiative [image-spec](https://github.com/opencontainers/image-spec/blob/master/annotations.md)


## Installation

To install Essex you can either clone this repository to any location on your machine and add it to your `$PATH`.
Or you can install with the following:  
```shell
\curl -sSL  https://raw.githubusercontent.com/utensils/essex/master/install.sh | bash -s
```

## Usage

List available Docker templates:
```shell
essex list
```  

Create a new project using a template:
```shell
essex new basic MyProject
```  

Update Essex:
```shell
essex update
```

