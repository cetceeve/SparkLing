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
model = mr.get_model("metro_delay_model", version=6)
model_dir = model.download()
model = joblib.load(model_dir + "/metro_delay_lstm.pkl")
# test_input = np.array([31,44,151,165,100,15,101,15,102,15,103,15,104,181,84,181,85,181,86,181,87,181,88,181,89,181,49,181,90,181,99,181,98,181,97,181,96,181,95,181,94,181,33])
# pred= model.inference(test_input)

# subscribe to new features on redis
r = redis.Redis(
    host="sparkling-redis",
    decode_responses=True,
)
sub = r.pubsub()
sub.subscribe("lstm-inference-features")


# upload new features to hopsworks
for msg in sub.listen():
    if msg["type"] != "message":
        continue

    # vehicle_id:<sos>,route,day,hour,stop,delta,stop,delta,stop,pad,stop,pad,stop,pad,pad,
    vehicle_id, sequence = tuple(msg["data"].split(":"))
    sequence = pd.read_csv(StringIO(sequence), header=None).to_numpy()[0]
    # 31,38,155,174,116,15,115,15,114,15,49,15,89,15,88,15,113,15,112,15,111,15,110,15,109,15,108,15,107,15,106,15,105,15,32,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33
    pred = model.inference(sequence)

    delay = 0
    output = [int(delay)]
    for token in pred[5::2]: # L[start:stop:step]
        if token > 30:
            # ignore non delay tokens
            delay += 0
        else:
            delay += token - 15

        output.append(int(delay))

    r.publish("realtime-with-metadata", json.dumps({"id": vehicle_id, "delay": output}))

