from Gaia.generalized_tool.GaiaDMML import *
import pika
import os
import json
import io
import time
import sys
import array

from dataclasses import dataclass
from typing import Optional, List

from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy import Integer, BLOB, Text, Float, Column, create_engine, update
from sqlalchemy.orm import sessionmaker

Base = declarative_base()
Session = None


RABBITMQ_ADDR = os.environ.get("RABBITMQ_ADDR") or "localhost"
CONNECTION_TIMEOUT = 10

_db_path = os.path.abspath(os.path.join(
    os.path.dirname(os.path.realpath(__file__)), "..", "gaia.db"))
DB_ADDR = os.environ.get("DATABASE_URL") or f"sqlite:////{_db_path}"
engine = create_engine(DB_ADDR)
Session = sessionmaker(bind=engine)


class Computation(Base):
    __tablename__ = 'computations'

    id = Column(Integer, primary_key=True)
    email = Column(Text)
    title = Column(Text)
    csv_file = Column(BLOB)
    hr_png = Column(BLOB)
    trimmed_png = Column(BLOB)
    distance_png = Column(BLOB)
    pm_png = Column(BLOB)
    correctly_clustered = Column(Integer)
    incorrectly_clustered = Column(Integer)
    accuracy = Column(Float)
    anomaly = Column(Integer)
    clusters = Column(Text)


def run_gaia(comp_id, db_scan, epsilon, cluster_size, filename):
    session = Session()
    this_comp: Computation = session.query(
        Computation).filter_by(id=comp_id).first()

    csv_file = io.BytesIO(this_comp.csv_file)
    distance_bytes = io.BytesIO()
    hr_bytes = io.BytesIO()
    trimmed_bytes = io.BytesIO()
    pm_bytes = io.BytesIO()

    df = create_df(csv_file)
    distance_plot(df, csv_file, filename, distance_bytes)
    hr_plots(df, csv_file, filename, hr_bytes)
    trimmed_df = trim_data(df)
    trimmed_hr(trimmed_df, csv_file, filename, trimmed_bytes)

    correctly_clustered, incorrectly_clustered, accuracy = (None, None, None)
    anomaly, actual_cluster_sizes = (None, None)
    if db_scan:
        # do DBScan stuff
        pm_plots(df, trimmed_df, csv_file, cluster_size, filename, pm_bytes)
        df_all_temp = source_id(df, int(cluster_size), int(epsilon))
        df_all, labels, n_clusters, n_noise = machine_learning(
            df, int(cluster_size), int(epsilon))
        anomaly = n_noise
        actual_cluster_sizes = amount(labels, n_clusters, n_noise)
        if n_clusters > 0:
            correctly_clustered, incorrectly_clustered, accuracy = compare_hr(
                trimmed_df, df_all, df_all_temp)

        this_comp.accuracy = accuracy
        this_comp.anomaly = anomaly
        this_comp.correctly_clustered = correctly_clustered
        this_comp.incorrectly_clustered = incorrectly_clustered
        this_comp.pm_png = pm_bytes.getvalue()

    this_comp.hr_png = hr_bytes.getvalue()
    this_comp.distance_png = distance_bytes.getvalue()
    this_comp.trimmed_png = trimmed_bytes.getvalue()

    if not actual_cluster_sizes is None and len(actual_cluster_sizes) > 0:
        this_comp.clusters = str(actual_cluster_sizes)

    distance_bytes.close()
    hr_bytes.close()
    trimmed_bytes.close()
    pm_bytes.close()

    session.commit()

    return (this_comp.id, this_comp.email)


def callback(ch, method, properties, body):
    request_info = json.loads(body)
    db_scan = request_info["db_scan"]
    epsilon = request_info["epsilon"]
    print(db_scan)
    cluster_size = request_info["cluster_size"]
    filename = request_info["title"]
    id, email = run_gaia(
        request_info["data_id"], db_scan, epsilon, cluster_size, filename)
    ch.basic_ack(method.delivery_tag)

    update_json_bytes = json.dumps({'id': id, 'email': email}).encode('ascii')
    ch.basic_publish(exchange="computation_updates", routing_key='', body=update_json_bytes)

    print("Done")


def timeout_connect():
    start = time.time()
    while (time.time() - start) < 10:
        try:
            return pika.BlockingConnection(
                pika.ConnectionParameters(host=RABBITMQ_ADDR)
            )
        except pika.exceptions.AMQPConnectionError:
            pass
    return None


def main():
    connection = timeout_connect()
    if connection is None:
        print("Could not connect to RabbitMQ server.", file=sys.stderr)
        exit(1)
    channel = connection.channel()
    channel.exchange_declare(
        exchange="computation_updates", exchange_type='fanout')

    channel.basic_consume(queue="gaia_input", on_message_callback=callback)
    channel.start_consuming()


if __name__ == '__main__':
    main()
