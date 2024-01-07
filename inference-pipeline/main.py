import redis
import hopsworks
import pandas as pd
import joblib
from io import StringIO
import numpy as np
import json

# connect to feature store
project = hopsworks.login(project="zeihers_mart")
mr = project.get_model_registry()
model = mr.get_model("metro_delay_model", version=15)
model_dir = model.download()
model = joblib.load(model_dir + "/metro_delay_lstm.pkl")

# subscribe to new features on redis
r = redis.Redis(
    host="sparkling-redis",
    decode_responses=True,
)
sub = r.pubsub()
sub.subscribe("lstm-inference-features")


cache = {}

# upload new features to hopsworks
for msg in sub.listen():
    if msg["type"] != "message":
        continue

    # re-send if no features included
    if not ":" in msg["data"]:
        vehicle_id = msg["data"]
        if vehicle_id in cache:
            r.publish("realtime-with-metadata", cache[vehicle_id])
        continue

    vehicle_id, sequence = tuple(msg["data"].split(":"))
    sequence = pd.read_csv(StringIO(sequence), header=None).to_numpy()[0]
    try:
        first_predicted_sequence = int((list(sequence).index(181) - 3) / 2) - 1
        pred = model.inference(sequence)
    except ValueError:
        # the vehicle has reached all stops, nothing to predict
        first_predicted_sequence = 100
        pred = sequence

    delay = 0
    output = []
    for token in pred[5::2]: # L[start:stop:step]
        if token > 30:
            # ignore non delay tokens
            delay += 0
        else:
            delay += token - 15

        output.append(int(delay))

    serialized = json.dumps({
        "id": vehicle_id,
        "delay": output,
        "first_predicted_sequence": first_predicted_sequence,
    })
    cache[vehicle_id] = serialized
    r.publish("realtime-with-metadata", serialized)

