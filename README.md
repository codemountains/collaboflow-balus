# Collaboflow Balus

**すべての申請書を削除します。実行しないことをおすすめします。** 

「目がぁぁぁぁあっ」ってなります。

## Getting Started

`sample.env` をコピーして `.env` を作成します。

```shell
cp sample.env .env
```

コラボフロー REST API を実行するために必要な情報を書き換えます。

- `BASE_URL` : コラボフロー REST API の URL
  - `https://{collaboflow url}/{instance name}/api/index.cfm/v1/` の形式で指定
- `USER_ID` : 管理者のユーザーID
- `API_KEY` : 発行した API キー

`cargo` コマンドで実行します。（それ相応の覚悟の上で実行してください）

```shell
cargo run
```

## Notes

大事なことなのでもう一度言います。

すべての申請書を削除します。実行しないことをおすすめします。

削除した申請書を復元する方法はありません。

## License

This project is licensed under the [MIT license](LICENSE).
