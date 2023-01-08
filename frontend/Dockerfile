FROM node:16

WORKDIR /usr/src/frontend
COPY . .

RUN yarn install
RUN yarn build

EXPOSE 8080

CMD yarn run run
