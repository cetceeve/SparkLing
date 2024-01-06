import redis
import hopsworks
import pandas as pd
import uuid
from io import StringIO


# connect to feature store
project = hopsworks.login(project="zeihers_mart")
fs = project.get_feature_store()
fg = fs.get_or_create_feature_group(
    name="metro",
    version=3,
    primary_key=["uuid"],
    description="Stockholm Metro delay prediction dataset"
)


# subscribe to new features on redis
r = redis.Redis(
    host="sparkling-redis",
    decode_responses=True,
)
sub = r.pubsub()
sub.subscribe("lstm-training-features")


# upload new features to hopsworks
buffer_df = None
for msg in sub.listen():
    if msg["type"] != "message":
        continue

    df = pd.read_csv(StringIO(msg["data"]), header=None)

    if buffer_df is None:
        buffer_df = df
    else:
        buffer_df = pd.concat([buffer_df, df])

    # write to hopsworks if buffer is full
    if len(buffer_df) >= 10:
        buffer_df.columns = buffer_df.columns.map(lambda i: f"feature_{i:02d}")
        buffer_df['uuid'] = [str(uuid.uuid4()) for _ in range(len(buffer_df.index))]
        fg.insert(buffer_df)
        buffer_df = None
