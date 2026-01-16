# Todo Web App
### written in Rust using axum, askama, htmx, SQLx (postgres), and tailwind.
### I may or may not change the design/ui. Will probably optimize it further as there are still things to improve

## About
I want to try and dive deep in web development in Rust. Why? because I can. Honestly, it's such a pain.
I don't know if it's a skill issue or it's really just that hard.

## Quick Start
```
git clone https://github.com/itlogsandwich/todo-web-app
cd todo-web-app

create .env file and ensure it contains DATABASE_URL, refer to .env.example

install tailwind cli
build the css, npx tailwindcss -i ./assets/input.css -o ./assets/output.css

cargo run -r 
```

Ensure that postgres has started and is listening.
You may manually prepare the database or allow sqlx to do it for you. 

```
systemctl start postgresql.service

psql -U [user] -D [database_name] -h localhost -W

sqlx database create // ensure that you have an .env file and the db url, refer to .env.example file
sqlx migrate add -r [migration name]
sqlx migrate run
```

