Gaia Web [![Build Status](https://travis-ci.com/Zedjones/GaiaWeb.svg?branch=master)](https://travis-ci.com/Zedjones/GaiaWeb)
==========
A web server and web worker implementation for [Gaia](https://github.com/lauramv1832/Gaia). Gaia Web utilizes RabbitMQ for job queuing, Python for the actual calculations, Rust for the web server (Actix), and React + Material UI for the frontend.

## Table of Contents
<!-- vim-markdown-toc GFM --> 
* [Usage](#usage)
* [Development](#licensing)

## Usage
The following environment variables can be set to configure the behavior of the application:

| Variable      | Default   |
|---------------|-----------|
| RABBITMQ_ADDR | 127.0.0.1 |
| DATABASE_URL  | gaia.db   |

The behavior of the logger can be configured using the `RUST_LOG`
variable as per the specification [here](https://docs.rs/env_logger/0.7.1/env_logger/).

Besides that, you can simply run the executable.

## Development
For proper development, you need Rust, npm, and preferably Docker installed.
To begin, go into the `frontend/` directory and run some npm commands:
```
$ cd frontend/
$ npm install && npm install -g npm-watch
$ npm run build
```
Next, run RabbitMQ however you want. Docker is an easy way:
```
$ docker run -d -p 15672:15672 -p 5672:5672 rabbitmq:3-management
```
The management container gives us a nice web interface at [http://localhost:15672](http://localhost:15672).


If you'll be working on the frontend, you can use `npm-watch` to automatically rebuild the frontend whenever you make any changes.
```
$ cd frontend/
$ npm run watch
```
You can also use the standard React build scripts, but the resources that the backend serves will not be updated and any calls from the frontend to the backend will fail. However, it does enable debugging of the React code so it might come in handy:
```
$ cd frontend/
$ npm run start
```