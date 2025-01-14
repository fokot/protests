# Protests app

Application where you can see the protests that are happening in the world.

## How to run

Setup postgres database and set correct url in `main.rs` file.

You can run postgres with docker:
```
docker run --name protests-postgres -e POSTGRES_USER=protests -e POSTGRES_PASSWORD=protests -e POSTGRES_DB=protests -p 5432:5432 -d postgres
```

To test if the database is running you can use `psql`:
```
psql -h localhost -U protests protests
```
Run app with
```
cargo run
```

and open [http://localhost:3000](http://localhost:3000) in your browser.

For development you can use `cargo watch` to automatically recompile the code:
```
cargo install cargo-watch
cargo watch -x run
```

## Project structure

```
├── asstes - static files server to browser
├── Settings.toml - configuration file
├── locales - fluent messages
├── migrations - postgres migrations
├── src - rust source code
└── templates - askama templates
```

## Technologies

* [axum](https://github.com/tokio-rs/axum) as webserver
* [askama](https://github.com/rinja-rs/askama) for html templates
* [sqlx](https://github.com/launchbadge/sqlx) as database client and migrations
* [fluent-rs](https://github.com/projectfluent/fluent-rs) for localization
* [tailwindcss](https://tailwindcss.com) for css


To build `tailwindcss` have `npm` v22+ installed and run:
```
npm run watch
```
It is needed only when you introduce new tailwind css class to html template.
If you change only rust code or don't use new css classes it is fine not to run it.
If there are changes always commit `main.css` file.