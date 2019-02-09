# d-sh

D-SH is tool, write in Rust, to launch **any** application in
docker in must transparency way.

By default, D-SH work with for **Ubuntu 18.04** with **Docker**.

I create this project cause I want run application without break my OS with new
installation or update (and also because it's funny).

```
Usage: d.sh COMMAND

A tool to container all your life

Options:
  -h, --help               Print this current help
  -v, --version            Print version information and quit

Commands:
  build (b)    Build container image
  check (chk)  List missing container image
  delete (rm)  Delete image
  init (i)     Initialize config file if not exists
  list (ls)    List all applications available
  run (r)      Run container
```

## Dependencies

You need install [Docker](https://docs.docker.com/install/) and Curl exe.

## How it's work

D-SH have only one binary file.

Because, many applications have same dependencies, D-SH use a Docker
*base image*. By default, this image inherit from Ubuntu 18.04 official
Docker image.

Each application inherit from this *base image*.

## Download applications

Applications are available in https://github.com/bubulemaster/d-sh-applications

## Install D-SH

Download last release and change add executable bit `chmod u+x ....`.

## Run D-SH

First initialize configuration folder `~/.d-sh` with:
```
./d-sh init
```

In this folder you can find three files:
 - `config.yml`: main config file,
 - `Dockerfile.hbs`: template of dockerfile,
 - `entrypoint.sh`: file include in dockerfile.

and two folder:
 - `applications`: contain YAML file of each application,
 - `download`: contain binaries of applications.

Now, build image:
```
./d.sh build atom
Building atom...
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   603    0   603    0     0    750      0 --:--:-- --:--:-- --:--:--   750
  0     0    0     0    0     0      0      0 --:--:--  0:00:01 --:--:--     0
Sending build context to Docker daemon  197.3MB
Step 1/5 : FROM d-base-image:v1.0.0
 ...
Successfully tagged run-atom:v1.27.2
```

First, D-SH download atom .deb binary file from official repository. This file
will be store in `download` folder.

NOTE: If the base image is not found, it's automatically generated.

Then, D-SH build an image.

Now we launch it:
```
$ ./d.sh run atom
Running atom...
Create container
9b20776a74df70075e0713fe5c5637242da5bbd816f1fc92923f7a24f121b8cb
```

For more informations, read [DOCUMENTATION](DOCUMENTATION.md) file.
