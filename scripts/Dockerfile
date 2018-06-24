FROM d-base-image:v1.0.0


ARG APPLICATION_DOWNLOADED_FILE_NAME
ARG DOWNLOADED_FILE_NAME_DEST

COPY $DOWNLOADED_FILE_NAME_DEST /tmp/$PROGRAM_TO_INSTALL

RUN apt-get update && \
    apt-get install -y \
      /tmp/$APPLICATION_DOWNLOADED_FILE_NAME && \
    rm -f /tmp/$APPLICATION_DOWNLOADED_FILE_NAME && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
