import pika
import json

connection = pika.BlockingConnection(
    pika.ConnectionParameters(host="localhost")
)
channel = connection.channel()
channel.queue_declare(queue="gaia_input")

def callback(ch, method, properies, body):
    request_info = json.loads(body)
    print(request_info["db_scan"])
    print(request_info["epsilon"])
    print(request_info["cluster_size"])
    print(request_info["data_id"])
    ch.basic_ack(delivery_tag=method.delivery_tag)

channel.basic_consume(queue="gaia_input", on_message_callback=callback)
channel.start_consuming()
