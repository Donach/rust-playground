# How to run
## Client
`cargo run --bin client <hostname> <port>`

## Server
`cargo run --bin server <hostname> <port>`

You can omit both hostname and port for each application, as it will default to running locally on port 11111.

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
Refactored few parts of code to better handle simple text and empty entry, which should correctly quit the client program
## 2. Logging
Using log and simple_logger to display nicer logging messages into stdout.