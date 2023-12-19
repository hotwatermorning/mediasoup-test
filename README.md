# SFU Test

## How to setup

* 3000/tcp, 3010/tcp, 50000-51000/tcp, 50000-51000/udp ポートを開ける
* 録画用に 12000-13000/tcp, 12000-13000/udp ポートが他のサービスに使われていない状態にする。
     * 他の範囲を使用したい場合は backend/src/recording.rs の RECORDING_PORT_MIN, RECORDING_PORT_MAX を変更する。
* バックエンドサーバーを公開するときの WebSocket URL (`ws://<host>:port/ws`) を ./frontend/.env.docker に設定する
* バックエンドサーバーの IP アドレスを backend/src/participant.rs の WebRtcTransportOptions に設定する

以上の設定を行ってバックエンドサーバーとフロントエンドサーバーを起動する。

```sh
cd backend
./run.sh
```

```sh
cd frontend
yarn
yarn dev
```

Linux 環境の場合は、以下のように docker コンテナを起動するのでも良い。
（`network_mode: host` を指定しているので、 Docker Desktop for macOS などの環境ではうまく動かない）

```sh
docker compose up -d
```
