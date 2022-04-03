# mood-broadcast-api
API for the Mood Broadcast project

# Environment files

To run the project, you need to create : 
- `Rocket.toml` based on the example file with username/password/url for your database
- `src/secret.key` using `head -c16 /dev/urandom > src/secret.key`

# Run the project without Docker

`cargo run` and you're good to go!

# Run the project using Docker

Run `docker-compose up` or build images yourself and run them :).
