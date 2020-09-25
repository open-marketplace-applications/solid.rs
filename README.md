# A SOLID implementation in Rust

## Architecture
The architecture is based on the description [here](https://rubenverborgh.github.io/solid-server-architecture/solid-architecture-v1-3-0.pdf). 
Authentication and authorization are cobined to one crate [auth](./auth).

- Auth
- CLI
- Core
- LDP
- Server
- Storage

## How to start
```bash
cargo run start
```

Visit [http://localhost:3000/](http://localhost:3000/)


## Interacting with the server
The server supports low-level interaction via HTTP methods, such as GET, PUT, HEAD, ...

Below, we provide several examples on how to interact with the server using curl

### `PUT`: Creating resources for a given URL

Create a plain text file:
```bash
$ curl -X PUT -H "Content-Type: text/plain" \
  -d "abc" \
  http://localhost:3000/myfile.txt
```

Visit [http://localhost:3000/myfile.txt](http://localhost:3000/myfile.txt)
