# Essex - Boilerplate for Docker Based Projects

## About

Project is still under active development and may not work as expected.  
**Pull requests always welcome**

Essex is a CLI utility written in bash to quickly setup consistent and clean Docker projects.

## Goals

 * Allow easy creation of Dockerfiles using best practices
 * Use a `Makefile` driven build workflow
 * Easily maintain consistency between projects using that use Docker
 * All images will have valid Labels/Annotations. See the Open Container Initiative [image-spec](https://github.com/opencontainers/image-spec/blob/master/annotations.md)  


This tool is intended to just lay down a starting point for each project. 
It will still require you to modify the Dockerfile and settings by hand just as you would any other Docker based project.

## Installation

To install Essex you can either clone this repository to any location on your machine and add it to your `$PATH`.
Or you can install with the following:  
```shell
\curl -sSL  https://raw.githubusercontent.com/utensils/essex/master/install.sh | bash -s
```

This will install essex into `~/.essex`.

## Usage

See usage with `essex --help`
```shell
 Essex master: Boilerplate for Docker Based Projects.
 License: MIT Copyright (c) 2019 Utensils Union

 Usage:
 	essex list
 	essex new <template> <ProjectName> [OPTION]...
 	essex update

 Options:
 	-v, --vendor [NAME]		Sets the vendor name of the project.

 Examples:
  	essex new basic MyApp
 	essex new basic MyApp --vendor utensils

```

Create a new project using a template:
```shell
essex new basic MyProject --vendor MyCompany
```  

With this new project in place you can start using the project instantly.
This project is pre-wired up with useful Make targets.
```shell
cd MyProject
make
make list
make test
```

Update Essex:
```shell
essex update
```

