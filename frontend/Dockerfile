FROM node:16

WORKDIR /usr/src/frontend

# Copy only the package.json and yarn.lock files to install dependencies
COPY package.json yarn.lock ./
RUN yarn install

# Copy the rest and build
COPY . .
RUN yarn build

EXPOSE 8080

CMD ["yarn", "run", "run"]
