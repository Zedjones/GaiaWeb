import pika
import json

connection = pika.BlockingConnection(
    pika.ConnectionParameters(host="localhost")
)
channel = connection.channel()
channel.queue_declare(queue="jobs_input")

def callback(ch, method, properies, body):
    file_name = f"./{method.delivery_tag}"
    with open(file_name, 'wb') as data_file:
        data_file.write(body)
    print("wrote file")
    ch.basic_ack(delivery_tag=method.delivery_tag)

channel.basic_consume(queue="jobs_input", on_message_callback=callback)
channel.start_consuming()
