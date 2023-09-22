FROM node:18

WORKDIR /usr/src/app

COPY package*.json ./

RUN npm install

COPY server.js server.js
COPY public/ public/

EXPOSE 8080

CMD [ "node", "server.js" ]