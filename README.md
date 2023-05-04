# listly

simple list app with multi-user concurrency and list sharing

## how to

run with

```bash
.\target\debug\listly.exe # no ".exe" off windows
# OR
cd .\backend\ cargo watch -q -c -w src/ -x run
# run database things with
create-migration.bat # or *.sh on linux
update-database.bat # or *.sh on linux, *-deploy.sh on deploy
```

## steps taken so far

add [diesel ORM](https://crates.io/crates/diesel) the mysql version (postgres would just not work for me ) for database things.
note: db pws and any deployment pws stored in [last pass](https://www.lastpass.com/)

```bash
cargo add diesel --no-default-features --features mysql
cargo install diesel_cli --no-default-features --features mysql
diesel setup #this creates the database and adds migration folder
```

logo colors

```json
{
  "Black olive":"353531",
  "Wheat":"e1cfa3",
  "Atomic tangerine":"e7a188",
  "Rosy brown":"cb9599",
  "Powder blue":"aeb9d5",
  "Vista Blue":"869dd9"
}
```

follow this [guide for oauth](https://codevoweb.com/how-to-implement-google-oauth2-in-rust/) to implement google Oauth in backend [repo here](https://github.com/wpcodevo/google-github-oauth2-rust)
