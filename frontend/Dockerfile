FROM node:lts-alpine

USER root
ADD . /usr/src/app
WORKDIR /usr/src/app

RUN addgroup -S poggit && adduser -S poggit -G poggit
RUN chown poggit:poggit . -R
USER poggit

RUN npm install -D
RUN npm run build
CMD npm run start
