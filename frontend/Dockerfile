FROM node:18-alpine

WORKDIR /app
COPY . .
COPY .env.docker .env

ARG ENV_FILE_NAME

RUN yarn install
RUN yarn build

EXPOSE 3000
CMD ["build/index.js"]