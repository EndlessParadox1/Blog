# Blog

> Just a demo blog implemented by axum.

## Structure

```
.
├── nginx
│   └── assets: Html and JS files.
├── docs: Config and DB-init files.
└── blog 
    └── src: Server-side app.
        ├── config: Init config from toml.
        ├── error: Unifying error.
        ├── form: Form submitted from client.
        ├── md: Converting md to html.
        ├── model: Data read from DB.
        ├── password: Hashing and verifying password.
        ├── rds: Managing session and `user_list`.
        ├── session: Managing session_id.
        ├── db
        │   ├── topic: Operation for `topics`.
        │   └── user: Operation for `users`.
        └── handler
            ├── auth: Protection for admin.
            ├── admin: Handling admin UI request.
            ├── front: Handling front UI request.
            ├── login: Handling login request.
            ├── register: Handling register request.
            └── topic: Handling detail of topic request.
```

## Background

Intend to develop a simple web application as full-stack developer so that learn and practice such as `Linux`, `PostgreSQL`, `Redis`, `Tokio/Axum`, `jQuery`, `Nginx` and `Docker` etc.

## Highlights

1. Session for auth.
2. Frontend and backend separation.
3. Cloudfare for cache, DNS and protection.
4. Dynamic and static separation & gateway served by Nginx.
5. Obey REST API norm.
6. Implement single page app.
7. Complete TLS for security and Http2 for performance.
8. Async client mode & async server mode implemented by Tokio/Axum.
9. Docker for automation, isolation and portability.

## Guide

1. Git clone the project.
2. Bash `docker compose up -d`.
3. You can also use image exited, i.e.`endlessparadox1/blog` and `endlessparadox1/nginx` in docker-compose.yml.
4. Remember, you must have a cert dir including SSL certificates before starting services.
5. All logs can be accessed with `docker logs [container_name]`.

## Contributing

Issues and Pull Requests are accepted. Feel free to contribute to this project.

## License

[MIT © EndlessParadox1.](./LICENSE)
