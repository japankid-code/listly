# listly

simple list app with multi-user concurrency and list sharing

## how to ___

to do

## steps taken so far

add [diesel ORM](https://crates.io/crates/diesel) the mysql version (postgres would just not work for me ) for database things. 

```bash
# note these must be run in command prompt, fails in git bash at least
cargo add diesel --no-default-features --features mysql
cargo install diesel_cli --no-default-features --features mysql
diesel setup #this creates the database and adds migration folder
```

follow this [guide for oauth](https://codevoweb.com/how-to-implement-google-oauth2-in-rust/) to implement google Oauth in backend [repo here](https://github.com/wpcodevo/google-github-oauth2-rust)

