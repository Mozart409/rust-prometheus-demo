# Rust Prometheus Demo

## Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Usage](#usage)

## About <a name = "about"></a>

This is a demo project to show how to use Prometheus with Rust and docker-compose.

## Getting Started <a name = "getting_started"></a>

Clone the repository and run `docker-compose up` to start the application. You can then access the Prometheus UI at `http://localhost:9090` and the application at `http://localhost:3000`.

### Prerequisites

You need to have docker, docker-compose and cargo installed. Optional is [just](https://github.com/casey/just)

## Usage <a name = "usage"></a>

Open the Prometheus UI and enter `http://localhost:9090` as the target. You can then use the query `http_requests_total` to see the number of requests made to the application.
