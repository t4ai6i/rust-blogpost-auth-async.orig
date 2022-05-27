* see
https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/

* database migrate
```
> diesel print-schema --database-url=auth0_demo.db > src/schema.rs
> diesel migration run --database-url=auth0_demo.db
```
