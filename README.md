# Blog

[![license](https://img.shields.io/github/license/EndlessParadox1/Blog?style=plastic)](LICENSE)[![code size](https://img.shields.io/github/languages/code-size/EndlessParadox1/Blog?style=plastic)]()[![platform](https://img.shields.io/badge/server-Linux-yellow?style=plastic)](https://en.wikipedia.org/wiki/Linux)[![platform](https://img.shields.io/badge/client-WebBrowser-lightblue?style=plastic)]()

>Just a demo blog implemented by axum.

## Project Structure

This project is organized as follows:

```
.
├── docs: Profile and database structure.
├── static: Html and JS for client.
└── src: Server-side application.
    ├── config: Init config from toml.
    ├── error: Unifying error.
    ├── form: Form submitted from client.
    ├── md: Converting md to html.
    ├── model: Data read from DB.
    ├── password: Hashing and verifying password.
    ├── rds: Managing session and user_list.
    ├── session: Managing session_id.
    ├── db
    │   ├── topic: Operation for topics.
    │   └── user: Operation for users.
    └── handler
        ├── auth: Protection for admin.
        ├── admin: Handling admin UI request.
        ├── front: Handling front UI request.
        ├── login: Handling login request.
        ├── register: Handling register request.
        └── topic: Handling detail of topic request.
    
```

## Background

Intend to develop a simple web application as full-stack developer 
so that learn and practice such as `Linux`, `PostgreSQL`, `Redis`, `Tokio/Axum`, `jQuery`, `Nginx` etc. 

## Highlights

1. Use session for auth.
2. Frontend and backend separation. 
3. Use Cloudfare for cache, DNS, protection and gateway.
4. Use Nginx for dynamic and static separation.
5. Use REST API style.
6. Implement single page app.
7. Use complete TLS for security.
8. Async server mode implemented by Tokio/Axum.


## Guide

1. Git clone the project.
2. Config your Redis and Postgresql.
3. Include `blog_nginx.conf` in .../nginx/conf.d.
4. Custom the `config.toml`.
5. Generate the exe with `cargo build --release`.
6. Finally, start the server with `nohup ./blog > .../out.blog 2 > &1 &`.

## Contributing

Issues and Pull Requests are accepted. Feel free to contribute to this project.

## Reference

* [漫游Axum](https://axum.rs/subject/roaming-axum)
* [使用axum构建博客系统](https://axum.rs/subject/blog)
* [Rust Course](https://course.rs/about-book.html)
* [PostgreSQL 13.1 手册](http://www.postgres.cn/docs/13/index.html)
* [jQuery 参考手册](https://www.w3school.com.cn/jquery/jquery_reference.asp)

## License

[MIT © EndlessParadox1.](./LICENSE)
