FROM node:18-alpine

COPY package.json package.json
COPY package-lock.json package-lock.json
COPY app.js app.js

RUN npm install

EXPOSE 3000

CMD npm run start
