FROM ubuntu

RUN apt update
RUN apt install libssl-dev sqlite3 -y

COPY ./target/release/gaia_web /var/gaia/gaia_web
COPY ./migrations /var/gaia/migrations

WORKDIR /var/gaia
CMD [ "/var/gaia/gaia_web" ]