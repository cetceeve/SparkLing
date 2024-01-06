import redis
import hopsworks
import pandas as pd
import torch
import joblib
from io import StringIO


# connect to feature store
project = hopsworks.login(project="zeihers_mart")
mr = project.get_model_registry()
model = mr.get_model("metro_delay_model", version=2)
print(model)
# model_dir = model.download()
# inference_model = joblib.load(model_dir + "/metro_delay_lstm.pkl")


# subscribe to new features on redis
r = redis.Redis(
    host="sparkling-redis",
    decode_responses=True,
)
sub = r.pubsub()
sub.subscribe("lstm-inference-features")


# upload new features to hopsworks
# buffer_df = None
for msg in sub.listen():
    if msg["type"] != "message":
        continue

    # vehicle_id:<sos>,route,day,hour,stop,delta,stop,delta,stop,pad,stop,pad,stop,pad,pad,
    training_id, sequence = tuple(msg["data"].split(":"))
    df = pd.read_csv(StringIO(sequence), header=None)

    print(training_id)
    print(df)

    # read sequence until first <skp> token

    # if buffer_df is None:
    #     buffer_df = df
    # else:
    #     buffer_df = pd.concat([buffer_df, df])

    # # write to hopsworks if buffer is full
    # if len(buffer_df) >= 10:
    #     buffer_df.columns = buffer_df.columns.map(lambda i: f"feature_{i:02d}")
    #     buffer_df['uuid'] = [str(uuid.uuid4()) for _ in range(len(buffer_df.index))]
    #     fg.insert(buffer_df)
    #     buffer_df = None
