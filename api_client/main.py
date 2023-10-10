from proto.gtfs_realtime_pb2 import *
from kafka import KafkaProducer
from time import sleep
from datetime import datetime
import grequests
import json
import sys
import os

def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)
def exception_handler(request, exception):
    eprint(f"client request error: {exception}")

TRAFIKLAB_GTFS_RT_KEY = os.getenv("TRAFIKLAB_GTFS_RT_KEY")

# operators adding realtime position feeds soon: jlt, vasterbotten
# TRANSPORT_AGENCIES = ["dt", "klt", "krono", "orebro", "skane", "sl", "ul", "vastmanland", "varm", "xt", "otraf"]
TRANSPORT_AGENCIES = ["sl", "ul"]

sleep(2)
producer = KafkaProducer(
    bootstrap_servers=["kafka:9092"],
    value_serializer=lambda v: json.dumps(v).encode('utf-8'),
)
last_update_timestamp = datetime.now()
eprint("bootstraping complete")

while True:
    now = datetime.now()
    requests = [
        grequests.get(
            f"https://opendata.samtrafiken.se/gtfs-rt-sweden/{agency}/VehiclePositionsSweden.pb?key={TRAFIKLAB_GTFS_RT_KEY}",
            headers={
                "accept": "application/octet-stream",
                "Accept-encoding": "gzip",
                "If-Modified-Since": f"{last_update_timestamp}",
                "If-None-Match": "bfc13a64729c4290ef5b2c2730249c88ca92d82d",
            }
        ) for agency in TRANSPORT_AGENCIES
    ]
    last_update_timestamp = now

    # asynchronously perform requests and process the responses as they come in
    for response in grequests.imap(requests, exception_handler=exception_handler):
        if response is None or not response.ok:
            eprint(f"client request error: {response}")
            continue
        feed_msg = FeedMessage()
        feed_msg.ParseFromString(response.content)
        for entity in feed_msg.entity:
            value = f"{entity.vehicle.vehicle.id},{entity.vehicle.trip.trip_id},{entity.vehicle.position.latitude},{entity.vehicle.position.longitude},{entity.vehicle.position.bearing},{entity.vehicle.position.speed}"
            timestamp_ms = entity.vehicle.timestamp * 1000
            producer.send(
                topic="realtime",
                value=value,
                timestamp_ms=timestamp_ms
            )

    # don't request more often than api can serve
    sleep(3)
