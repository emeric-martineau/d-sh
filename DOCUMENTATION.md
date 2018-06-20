# How add application

To add a new application, add file in `program` folder. Filename (without `.sh`
extension) is the name of application.

In this file, we need some environment variables:
```
DOWNLOADED_FILE_NAME: name of file that store in download folder (e.g. atom.deb)
URL                 : url to download file (e.g. https://.../atom/1.27.2/release.deb)
IMAGE_DOCKER        : name of docker image to be create (e.g. run-atom:v1.27.2)
EXTRA_BUILD_ARG     : put here some extra docker build args
DEPENDENCIES        : list of dependencies that you would like install in base image
COMMAND_LINE        : command to run application in container (e.g /user/bin/atom)
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

## In nutshell

### Docker image entrypoint

The entrypoint of base image point to `entrypoint.sh` script. This file create
user, group and home folder of user that launch container. This script set also
owner of home folder. Finally, script run application with a substitute user and
group ID.

### Home mapping

Full home user that launch application are mount in container home user's folder.
