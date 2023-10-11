FROM node:18-alpine

WORKDIR /app
COPY . .

RUN yarn install

RUN cp .env.docker .env
RUN yarn build

EXPOSE 3000
CMD ["build/index.js"]