# SFU Test

## How to setup

* 3000/tcp, 3010/tcp, 50000-51000/tcp, 50000-51000/udp ポートを開ける
* バックエンドのサーバーを公開するときの WebSocket URL を ./frontend/.env.docker に設定する
* サーバー IP アドレスを backend/src/participant.rs の WebRtcTransportOptions に設定する

以上の設定を行って docker コンテナを起動する。

```sh
docker-compose up -d
```
