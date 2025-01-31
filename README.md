# Rust Way

[![Tests](https://github.com/humangrass/rust-way/actions/workflows/rust.yml/badge.svg)](https://github.com/humangrass/rust-way/actions/workflows/rust.yml)

Collection of projects on the way to Rust.

- **bartender**

  Authentication service using JWT middleware.

- **todo**

  [//]: # (TODO: todo service)
  Task management service. The project is as simple as possible and would be improved in the future.

- **todo-cli**

  [//]: # (TODO: todo-cli service)
  CLI service for exploring the incredible [clap](https://github.com/clap-rs/clap). May be improved or removed in the
  future.

## Starting services

The **bartender** and **todo** services can be started using [Docker Compose](https://docs.docker.com/compose/). To do
this, run in the root of the project:

```bash
docker-compose up -d
```

For more information on setting it up, see [docker-compose.yml](docker-compose.yml).

## API Documentation

After the containers have been successfully launched, the documentation pages are available at the following addresses:

- bartender: http://localhost:3001/docs
- todo: http://localhost:3000/docs

## Conclusion

The project demonstrates the basic principles of working with Docker and Rust services.
If you have any questions or suggestions for improvement, create an issue in the repository.
