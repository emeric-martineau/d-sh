# Config file of D-SH format

```
---
download_dir: "dwn"
applications_dir: "app"
dockerfile:
  from: "ubuntu:18.04"
  tag: "d-base-image:v1.0.0"
# This line is optional. By default use /tmp
tmp_dir: "~/.tmp"
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

To add a new application, add file in `applications_dir` folder. Filename (without `.yml`
extension) is the name of application.

In this file, we need some properties:
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

# Hack D-SH

## Change Ubuntu version or image base

By default, when you initialize D-SH, 4 Dockerfile are created.

If you want change Ubuntu version, edit `scripts/Dockerfile.base` file and
change line `FROM ubuntu:18.04`.

A last file is entrypoint script `entrypoint.sh`.

## D-SH behind proxy

To allow Ubuntu image to download dependencies, edit `Dockerfile.base`
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

TODO move into other file to explain how developp

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
