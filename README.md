# rust-redis-pooling-example
Example of using Redis connection pool with Rocket's `rocket_contrib::databases`

This repo is spawn out of me toying with rust for some experiments.

Hopefully this repo and these notes would be useful to someone else later.

## This example makes a few assumptions:
* the redis server is running on `127.0.0.1:6379`
* there is a string value for key `"abc"`, if you are not expecting a value, remember to use `Option`

## Learning points
* rocket_contrib::databases is fixed with a specific version of redis - `0.8.0` as of writing. This means it is best to use `rocket_contrib::databases::redis::Connection` instead of `redis::Connection` directly.
* It is required to expose redis command via `use rocket_contrib::databases::redis::Commands;` not with `redis::Command`
