# d-sh

D-SH is tool (purely write in shell script) to launch **any** application in
docker in must transparancy way.

By default, D-SH work only for **Ubuntu 18.04** with **Docker**.

I create this projet cause I want run application without break my OS with new
installation or update (and also because it's funny).

```
Usage: d.sh COMMAND

A tool to container for all your life

Options:
  -c, --check              List missing container image
  -h, --help               Print this current help
  -l, --list               List all program avaible
  -v, --version            Print version information and quit

Commands:
  build    Build container image
  delete   Delete image
  run      Run container
```

## How it's work

D-SH have on main script `d.sh` and launch others scripts called *command* that
can be found in `scripts/command`.

Because, many applications have same dependencies, D-SH use a Docker
*base image*, build with `scripts/Dockerfile.base` file. This image inherit
from Ubuntu 18.04 official Docker image.

Each application inherit from this *base image*.

## Install D-SH

To install D-SH, just clone this repository.

## Run D-SH

First, list all applications available:
```
$ ./d.sh -l
atom
gitkraken
keeweb

```

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
