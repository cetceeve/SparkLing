import pandas as pd
import numpy as np
import geopandas as gpd


aggregate_df = pd.read_csv("./sample_data/sweden_aggregated_metadata.csv")
stops_df = pd.read_csv("./sample_data/static.gtfs/stops.txt")
stop_times_df = pd.read_csv("./sample_data/static.gtfs/stop_times.txt")

# prepare df of stop sequences for each route
stop_seq_df = aggregate_df[["route_id", "trip_id", "direction_id"]] \
    .merge(stop_times_df, on="trip_id") \
    .drop_duplicates(subset=["route_id", "direction_id", "stop_sequence"]) \
    .merge(stops_df[["stop_id", "stop_name", "stop_lat", "stop_lon"]])
stop_seq_df = stop_seq_df[["stop_id", "stop_name", "stop_lat", "stop_lon", "stop_sequence", "direction_id", "route_id", "shape_dist_traveled"]]
# print(stop_seq_df)
# exit()

df = pd.read_csv(
    "./sample_data/2023-12-03T22_45_32-rt-data.csv.gz",
    dtype={
        "direction_id": pd.Int64Dtype(),
        "shape_id": pd.Int64Dtype(),
        "route_type": pd.Int64Dtype(),
        "trip_id": pd.Int64Dtype(),
    }
)

# remove rows without necessary metadata
df = df.dropna(subset=["trip_id"])

# convert timestamp
df["datetime"] = pd.to_datetime(df["timestamp"], unit="s")
df["weekday"] = df["datetime"].dt.dayofweek
df["hour"] = df["datetime"].dt.hour

# we don't have the route_id on the events, so we need to get it back
df["compound_route_id"] = df[["agency_name", "route_short_name", "route_long_name"]].astype(str).agg("|".join, axis=1)
aggregate_df["compound_route_id"] = aggregate_df[["agency_name", "route_short_name", "route_long_name"]].astype(str).agg("|".join, axis=1)
df = df.merge(aggregate_df[["compound_route_id", "route_id"]].drop_duplicates(), on="compound_route_id", how="inner")

# make sure we only have one trip per vehicle in this batch, to simplify further processing
counts_df = df[["id", "trip_id", "lng"]].groupby(by=["id", "trip_id"], as_index=False).count()
counts_df = counts_df.sort_values(by="lng").drop_duplicates(subset=["id"])
df = df[df["trip_id"].isin(counts_df["trip_id"])]

# attach stop information for events at stops
# TODO: is 100m the correct distance threshold? - seems okay at first glance at the output - 30m was too low
tmp_df = df.merge(stop_seq_df, on=["route_id", "direction_id"], how="left")
tmp_df["distance"] = gpd.GeoSeries.from_xy(tmp_df["lng"], tmp_df["lat"], crs="EPSG:4326").to_crs(epsg=3763).distance(gpd.GeoSeries.from_xy(tmp_df["stop_lon"], tmp_df["stop_lat"], crs="EPSG:4326").to_crs(epsg=3763))
tmp_df = tmp_df[tmp_df["distance"] <= 100.0].drop_duplicates(subset=["id", "timestamp"])
tmp_df = tmp_df[["id", "timestamp", "stop_id", "stop_name", "stop_lat", "stop_lon", "stop_sequence", "distance", "shape_dist_traveled"]]
df = df.merge(tmp_df, on=["id", "timestamp"], how="left")

# find previous vehicle on same route
vehicle_df = df[["id", "route_id", "direction_id", "timestamp", "stop_id"]]
vehicle_df = vehicle_df.dropna(subset=["stop_id"])
vehicle_df = vehicle_df.sort_values(by="timestamp") # sort and drop, to only keep arrival times at stops
vehicle_df = vehicle_df.drop_duplicates(subset=["id", "stop_id"], keep="first")
vehicle_df = vehicle_df.merge(vehicle_df, on=["route_id", "direction_id", "stop_id"], how="inner")
vehicle_df = vehicle_df[vehicle_df["id_x"] != vehicle_df["id_y"]]
vehicle_df = vehicle_df[vehicle_df["timestamp_x"] > vehicle_df["timestamp_y"]]
vehicle_df["time_delta"] = vehicle_df["timestamp_x"] - vehicle_df["timestamp_y"]
# we use this sort key to get the directly previous vehicle, but specifically keep the row with the most recent stop that both have reached
# so that we get the most recent time_delta
vehicle_df["sort_key"] = vehicle_df["time_delta"] - (vehicle_df["timestamp_x"] * 100000)
# vehicle_df = vehicle_df[vehicle_df["time_delta"] > 10]
vehicle_df = vehicle_df.sort_values(by="sort_key")
vehicle_df = vehicle_df.drop_duplicates(subset=["id_x"], keep="first")
vehicle_df = vehicle_df.rename(columns={"id_x": "id", "id_y": "prev_vehicle_id"})
df = df.merge(vehicle_df[["id", "prev_vehicle_id", "time_delta"]], on="id", how="left")
# TODO: we don't need the most recent time_delta globally, but the locally most recent one :(

# attach labels to data (actual time to stop)
# df = df.dropna(subset="time_delta")


# event metadata, wochentag, tageszeit stunde, arrival time prev vehicle, time_delta to prev vehicle at last stop
features = []


print(df[["id", "distance", "stop_sequence", "lng", "lat", "route_id", "direction_id", "shape_dist_traveled", "datetime", "time_delta"]])
# df[df["id"] == 9031008020600368][["datetime", "timestamp", "time_delta", "stop_sequence", "stop_name"]].sort_values(by="timestamp").to_csv("test.csv", index=False)
