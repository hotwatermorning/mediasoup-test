# SFU Test

## How to setup

```sh
cd webrtc-test
git submodule update --init --recursive

# デプロイ環境に合わせて coturn の設定を更新する
cd turn-server
cp turnserver.conf.example turnserver.conf
vim turnserver.conf

# 必要に応じて TLS 証明書を指定する
export COTURN_CERT_PEM=/path/to/cert.pem
export COTURN_PKEY_PEM=/path/to/pkey.pem

docker-compose up -d --build
```
