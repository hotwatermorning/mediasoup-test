# FROM node:18-alpine
#
# RUN apk update && apk add bash
#
# WORKDIR /home/frontend

FROM node:18-alpine as builder

WORKDIR /app
COPY . .

ARG ENV_FILE_NAME

RUN yarn install
RUN yarn build

FROM node:18-slim

COPY --from=builder /app/package.json package.json
COPY --from=builder /app/node_modules node_modules
COPY --from=builder /app/build build
COPY --from=builder /app/.env.docker .env


EXPOSE 3000
CMD ["build/index.js"]