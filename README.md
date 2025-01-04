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