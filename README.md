# server



![cross build](https://github.com/customize-game/server/workflows/cross%20build/badge.svg)




# How to Use

1. build images

   ```shell
   make build
   ```

2.  Make `.env` file. Sample .env file is bellow.

   ```shell
   DB_USER=USER
   DB_PASSWORD=USER
   DB_PORT=3306
   DB_NAME=player_info
   SERVER_PORT=5000
   ```

   

3. create container from build image

   ```shell
   make run
   ```

   

4. execute an interactive bash on the container

   ```shell
   make exec
   ```

5.  run `cargo run` in container

   ```shell
   cargo run
   ```

   



<br />

# Help

```shell
make help
```

# diesel

```shell
make exec_rust
cargo install diesel_cli --no-default-features --features mysql
diesel migration run
```



 