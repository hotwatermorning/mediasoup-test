FROM node:18-alpine

WORKDIR /app
COPY . .

RUN yarn install
RUN yarn build
RUN cp .env.docker .env

EXPOSE 3000
CMD ["build/index.js"]