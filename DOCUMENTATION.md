# Config file of D-SH format

```
---
download_dir: "dwn"
applications_dir: "app"
```

# Config file of application format

```
---
download_filename: "...."
url: "..."
image_name: "..."
dependencies:
  - ...
  - ...
cmd_line: "...."
cmd_line_args:
 - ...
 - ...
interactive: true | false
```

# Support installation format

D-SH support install file from:
 * `.deb`
 * `.tar.gz`
 * `.tgz`
 * `.tar.bz2`
 * `.tar.xz`
 * native linux distribution repository

# How add application

To add a new application, add file in `program` folder. Filename (without `.sh`
extension) is the name of application.

In this file, we need some environment variables:
```
APPLICATION_DOWNLOADED_FILE_NAME  : name of file that store in download folder (e.g. atom.deb)
APPLICATION_URL                   : url to download file (e.g. https://.../atom/1.27.2/release.deb)
APPLICATION_IMAGE_DOCKER          : name of docker image to be create (e.g. run-atom:v1.27.2)
APPLICATION_DEPENDENCIES          : list of dependencies that you would like install in base image
APPLICATION_COMMAND_LINE          : command to run application in container (e.g /user/bin/atom)
APPLICATION_IPC_HOST              : set "true" if need ipc host. Some X11 application need this
APPLICATION_SKIP_CHECK_REDOWNLOAD : if we want never check new version. Example for Postman, If-modified-date not supported (set to "true")
APPLICATION_INTERACTIVE           : run application in console (set to "true")
```

# Hack D-SH

## Change Ubuntu version or image base

If you want change Ubuntu version, edit `scripts/Dockerfile.base` file and
change line `FROM ubuntu:18.04`.

## D-SH behind proxy

To allow Ubuntu image to download dependencies, edit `scripts/Dockerfile.base`
file and add juste after line `ARG DEPENDENCIES_ALL`:
```
ENV ALL_PROXY socks://xx.xx.xx.xx:3128/
ENV FTP_PROXY http://xx.xx.xx.xx:3128/
ENV HTTPS_PROXY http://xx.xx.xx.xx:3128/
ENV HTTP_PROXY http://xx.xx.xx.xx:3128/
ENV all_proxy socks://xx.xx.xx.xx:3128/
ENV ftp_proxy http://xx.xx.xx.xx:3128/
ENV http_proxy http://xx.xx.xx.xx:3128/
ENV https_proxy http://xx.xx.xx.xx:3128/

RUN echo 'Acquire::http::Proxy "http://xx.xx.xx.xx:3128";' >> /etc/apt/apt.conf && \
    echo 'Acquire::https::Proxy "http://xx.xx.xx.xx:3128";' >> /etc/apt/apt.conf
```

Rebuild base image and all applications images

## Add a new command

To add new command, just create a file in `scripts/command` folder
named `command-XXXX.sh` where `XXXX` is the name of command.

Command name must be in **lowercase**.

The `command-XXXX.sh` must be contain their environment variables:
```
COMMAND_DESCRIPTION : description of command display in help
COMMAND_MIN_ARGS    : minimum args that command need
COMMAND_MAX_ARGS    : maximum args (-1 for no maximum)
```

and need have a function called `command_XXXX`.

In the `command_XXXX.sh` file, you receive two environment variable:
```
PROGRAM_NAME : this is the first parameter of command line. Example './d.sh build atom' this is atom
COMMON_FILE  : this is name of application file
```

and can use `RETURN_CODE` to set a exit code value of `d.sh` script.


## In nutshell

### Docker image entrypoint

The entrypoint of base image point to `entrypoint.sh` script. This file create
user, group and home folder of user that launch container. This script set also
owner of home folder. Finally, script run application with a substitute user and
group ID.

### Home mapping

Full home user that launch application are mount in container home user's folder.

### Dockerfiles

```
Dockerfile.base           : Base image for all application
Dockerfile.from-deb-file  : Use to build image of application if file is a *.deb
Dockerfile.from-package   : Use to build image of application if file is a package in linux distribution
Dockerfile.from-tgz-file  : Use to build image of application if file is a *.tar.gz, *.tgz, *.tar.bz2, *.tar.xz
```

### Add tests

For more informations, read [TESTS](TESTS.md) file.
