# How to run
## Client
`cargo run --bin client <hostname> <port> <uid>`

## Server
`cargo run --bin server <hostname> <port>`

You can omit all arguments for each application, as it will default to running locally on port 11111, and client would generate a new UID.

# Install sqlx-cli
`cargo install sqlx-cli`

# In case you deleted db, create it with 
`cargo sqlx database create`

# Sending data from client to server
## Message
You can send any arbitrary message to the server by just typing to console once client is started.

## File
You can send any file by using `.file` command with full (or relative) path to given file:
`.file <full_file_path>`

## Image
Send any image over to server, and have it converted to PNG automatically using `.image` command:
`.image <full_image_path>`

## Quit
You can exit the client by typing `.quit` or submitting empty command/message.

# Main changes from previous version
## 1. Refactoring
Refactored to use tokio async framework on both server and client side.
## 2. Database
Added support for sqlite database, which is used to store messages and client IDs
## 3. Better error handling
Previous version had still issues with client reconnecting that should be solved now
In general, no program should panic

# L16 HW Changes
- Moved db_client.rs from server to library
- Moved input_handler.rs from client to library
- Added tests for input_handler and db_client

# Metrics via Prometheus
## Setup
The setup involves Prometheus for metrics tracking. If you want to run the Prometheus, make sure to update IP in `prometheus.yml` appropriately to your host where the metrics server will run (default is <docker_local_ip>:8001 - however you need to use your "docker" IP of your computer if you run it locally)

To change the IP, change it in `counters.rs` in server crate, and also in `prometheus.yml`.

To start, simply run in root of this project:
`docker compose up`
and visit `http://localhost:9090`