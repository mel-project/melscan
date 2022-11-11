FROM node:latest

RUN apt update
RUN apt -y install curl wget nmap

WORKDIR /tmp
ENV BATS_VERSION="1.7.0"
RUN wget "https://github.com/bats-core/bats-core/archive/refs/tags/v${BATS_VERSION}.tar.gz"
RUN tar -xf "v${BATS_VERSION}.tar.gz"

WORKDIR "bats-core-${BATS_VERSION}"
RUN ./install.sh /usr/local

WORKDIR /tmp
RUN rm -rf "v${BATS_VERSION}.tar.gz"
RUN rm -rf "bats-core-${BATS_VERSION}"

COPY melscan-backend /usr/local/bin/melscan-backend
RUN chmod +x /usr/local/bin/melscan-backend

COPY themelio-node /usr/local/bin/themelio-node
RUN chmod +x /usr/local/bin/themelio-node

COPY build /var/www/melscan-frontend/build
COPY package.json /var/www/melscan-frontend/
WORKDIR /var/www/melscan-frontend
RUN yarn install


COPY ci.bats /tmp/ci.bats

COPY run.sh /usr/local/bin/run.sh

WORKDIR /

EXPOSE 3000

ENTRYPOINT ["run.sh"]