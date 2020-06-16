FROM node AS frontend-builder
WORKDIR /opt/frontend
COPY ./frontend/ ./
RUN npm install && npm run build

FROM rust as backend-builder
WORKDIR /opt/gaia_web
COPY ./src ./
RUN cargo build --release

FROM ubuntu

RUN apt update
RUN apt install libssl-dev sqlite3 -y

COPY --from=backend-builder /opt/gaia_web/gaia_web /var/gaia/gaia_web
COPY --from=frontend-builder /opt/frontend/frontend/build/ /var/gaia/frontend/build/
COPY ./migrations /var/gaia/migrations

WORKDIR /var/gaia
CMD [ "/var/gaia/gaia_web" ]