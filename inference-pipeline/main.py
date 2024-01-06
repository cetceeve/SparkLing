import redis
import hopsworks
import pandas as pd
import joblib
from io import StringIO
import numpy as np

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
# buffer_df = None
for msg in sub.listen():
    if msg["type"] != "message":
        continue

    # vehicle_id:<sos>,route,day,hour,stop,delta,stop,delta,stop,pad,stop,pad,stop,pad,pad,
    training_id, sequence = tuple(msg["data"].split(":"))
    print(training_id)
    df = pd.read_csv(StringIO(sequence), header=None)
    print(df)
    sequence = df.to_numpy()[0]
    print(sequence)

    pred = model.inference(sequence)
    print(pred)




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
