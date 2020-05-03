FROM ubuntu

RUN apt update
RUN apt install libssl-dev -y

COPY ./target/release/actix_test /var/gaia/gaia

CMD [ "/var/gaia/gaia" ]