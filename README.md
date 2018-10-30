# Rust-webapp-starter 

> The Extend project
>  **[ruster](https://github.com/rustlang-cn/ruster)**
>  **[rustlang-cn/Rust-webapp-starter: Rust single page webapp written in actix-web with vuejs.](https://github.com/rustlang-cn/Rust-webapp-starter)**

Rust single page webapp written in [actix-web](https://github.com/actix/actix-web) with vuejs.

* Async stable Actix-web framework
* diesel, postgresql r2d2
* SPA CORS JWT
* Vuejs

## when development 

```bash
$ docker-compose up -d
$ docker-compose exec api diesel setup
$ cargo run
```

then open browser 'http://localhost:18080/'
