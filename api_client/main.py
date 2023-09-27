from proto.gtfs_realtime_pb2 import *
from kafka import KafkaProducer
from time import sleep
from datetime import datetime
import requests
import json
import sys
import os

def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)

sleep(2)
producer = KafkaProducer(
    bootstrap_servers=["kafka:9092"],
    value_serializer=lambda v: json.dumps(v).encode('utf-8'),
)
last_update_timestamp = datetime.now()
eprint("bootstraping complete")

while True:
    sleep(3)

    now = datetime.now()
    resp = requests.get(
        f"https://opendata.samtrafiken.se/gtfs-rt-sweden/ul/VehiclePositionsSweden.pb?key={os.getenv('TRAFIKLAB_REALTIME_KEY')}",
        headers={
            "accept": "application/octet-stream",
            "Accept-encoding": "gzip",
            "If-Modified-Since": f"{last_update_timestamp}",
            "If-None-Match": "bfc13a64729c4290ef5b2c2730249c88ca92d82d",
        }
    )
    last_update_timestamp = now

    # producer.send(topic="realtime", key="ul", value=resp.content)

    feed_msg = FeedMessage()
    feed_msg.ParseFromString(resp.content)


    for entity in feed_msg.entity:
        # key = entity.id
        value = {
            "vehicle_id": entity.vehicle.vehicle.id,
            "position": {
                "lat": entity.vehicle.position.latitude,
                "long": entity.vehicle.position.longitude,
                "bearing": entity.vehicle.position.bearing,
                "speed": entity.vehicle.position.speed,
            }
        }
        timestamp_ms = entity.vehicle.timestamp * 1000
        # print(f"key: {ul}")
        # print(f"value: {value}")
        # print(f"ts: {timestamp_ms}")
        producer.send(
            topic="realtime",
            value=value,
            # key="ul",
            timestamp_ms=timestamp_ms
        )
