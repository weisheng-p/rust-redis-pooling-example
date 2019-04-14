# rust-test-redis-pooling
Example of using Redis connection pool with Rust

This example makes a few assumptions:
* the redis server is running on `127.0.0.1:6379`
* there is a string value for key `"abc"`, if you are not expecting a value, remember to use `Option`
