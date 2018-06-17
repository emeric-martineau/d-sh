FROM d-base-image:v1.0.0


ARG PROGRAM_TO_INSTALL
ARG DOWNLOADED_FILE_NAME_DEST

COPY $DOWNLOADED_FILE_NAME_DEST /tmp/$PROGRAM_TO_INSTALL

RUN apt-get update && \
    apt-get install -y \
      /tmp/$PROGRAM_TO_INSTALL && \
    rm -f /tmp/$PROGRAM_TO_INSTALL && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
