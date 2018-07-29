APPLICATION_DOWNLOADED_FILE_NAME='chrome.deb'
APPLICATION_URL="https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb"
APPLICATION_IMAGE_DOCKER="run-chrome:latest"
APPLICATION_DEPENDENCIES="ca-certificates fonts-liberation libappindicator3-1libasound2libatk-bridge2.0-0libatk1.0-0libc6libcairo2,
                          libcups2libdbus-1-3libexpat1libgcc1libgdk-pixbuf2.0-0libglib2.0-0libgtk-3-0libnspr4libnss3libpango-1.0-0,
                          libpangocairo-1.0-0libstdc++6libx11-6libx11-xcb1libxcb1libxcomposite1libxcursor1libxdamage1libxext6,
                          libxfixes3libxi6libxrandr2libxrender1libxss1libxtst6lsb-releasewgetxdg-utils libu2f-udev"
# Ok, not good. Chrome requiered privilege
APPLICATION_COMMAND_LINE="/usr/bin/google-chrome-stable --no-sandbox"
