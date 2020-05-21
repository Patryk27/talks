FROM node:13-stretch-slim

# Install Chromium's dependencies
RUN apt update && apt install -y \
  gconf-service \
  libxext6 \
  libxfixes3 \
  libxi6 \
  libxrandr2 \
  libxrender1 \
  libcairo2 \
  libcups2 \
  libdbus-1-3 \
  libexpat1 \
  libfontconfig1 \
  libgcc1 \
  libgconf-2-4 \
  libgdk-pixbuf2.0-0 \
  libglib2.0-0 \
  libgtk-3-0 \
  libnspr4 \
  libpango-1.0-0 \
  libpangocairo-1.0-0 \
  libstdc++6 \
  libx11-6 \
  libx11-xcb1 \
  libxcb1 \
  libxcomposite1 \
  libxcursor1 \
  libxdamage1 \
  libxss1 \
  libxtst6 \
  libappindicator1 \
  libnss3 \
  libasound2 \
  libatk1.0-0 \
  libc6 \
  ca-certificates \
  fonts-liberation \
  lsb-release \
  xdg-utils \
  wget

# Install Puppeteer
RUN npm install -g --unsafe-perm puppeteer

# Install asciidoctor-pdf.js's dependencies
RUN apt install -y git

# Install asciidoctor-pdf.js
RUN npm install -g --unsafe-perm \
  @asciidoctor/core \
  asciidoctor-pdf \
  asciidoctor-kroki

# Prepare an unprivileged user
RUN groupadd -r user \
 && useradd -r -g user -G audio,video user \
 && mkdir -p /home/user/Downloads \
 && chown -R user:user /home/user

USER user
WORKDIR /mnt
ENTRYPOINT ["asciidoctor-pdf"]
