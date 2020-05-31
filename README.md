Gaia Web [![Build Status](https://travis-ci.com/Zedjones/GaiaWeb.svg?branch=master)](https://travis-ci.com/Zedjones/GaiaWeb)
==========
A web server and web worker implementation for [Gaia](https://github.com/lauramv1832/Gaia). Gaia Web utilizes RabbitMQ for job queuing, Python for the actual calculations, Rust for the web server (Actix), and React + Material UI for the frontend.

## Table of Contents
<!-- vim-markdown-toc GFM --> 
* [Usage](#usage)
* [Development](#development)
  * [Web Backend/Frontend](#web-backendfrontend)
  * [Worker](#worker)

## Usage
The following environment variables can be set to configure the behavior of the application:

| Variable      | Default   |
|---------------|-----------|
| RABBITMQ_ADDR | 127.0.0.1 |
| DATABASE_URL  | gaia.db   |

The behavior of the logger can be configured using the `RUST_LOG`
variable as per the specification [here](https://docs.rs/env_logger/0.7.1/env_logger/).

## Development

### Web Backend/Frontend

You need Rust, npm, and preferably Docker installed.
To begin, go into the `frontend/` directory and run some npm commands:
```
$ cd frontend/
$ npm install
$ npm start
```
Next, run RabbitMQ however you want. Docker is an easy way:
```
$ docker run -d -p 15672:15672 -p 5672:5672 rabbitmq:3-management
```
The management container gives us a nice web interface at [http://localhost:15672](http://localhost:15672).

Finally, run the web server using `cargo`:
```
$ cargo run
```
Now, you can access the web server at [http://localhost:8080](http://localhost:8080).

### Worker

The same environment variables also apply to the Python worker. However, the default value for `DATABASE_URL` is `../gaia.db` instead. To run it, you'll need Python 3.6+, pip, and the dependencies. You can install the dependencies as follows:
```
$ cd worker
$ pip3 install --user -r requirements.txt
```

Now, you can run the worker with `python3` after initializing the Gaia submodule:
```
$ git submodule update --init
$ cd worker
$ python3 WebWorker.py
```

### Recommended Environment
To quickly and effective write code for this project, [VS Code](https://code.visualstudio.com/) is recommended with the following extensions:
 - SQLite
 - rust-analyzer
 - Search crates.io
 - Python
 - crates
 - Docker
 - Docker Compose
 - ESLint