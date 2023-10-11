FROM node:18-alpine

WORKDIR /app
COPY . .

RUN yarn install
RUN yarn build

EXPOSE 3000
CMD ["build/index.js"]