import functions_framework
from zipfile import ZipFile
from urllib.request import urlopen, Request
import pandas as pd
import numpy as np
from io import BytesIO
from google.cloud import storage
from datetime import datetime
import os

@functions_framework.cloud_event
def aggregate_gtfs_static_sweden(cloud_event):
    TRAFIKLAB_GTFS_STATIC_KEY = os.environ.get("TRAFIKLAB_GTFS_STATIC_KEY", "Specified environment variable is not set.")
    print(TRAFIKLAB_GTFS_STATIC_KEY)
    print("downloading gtfs static sweden...")
    req = Request(f"https://opendata.samtrafiken.se/gtfs-sweden/sweden.zip?key={TRAFIKLAB_GTFS_STATIC_KEY}")
    req.add_header("accept", "application/octet-stream")
    req.add_header("accept-encoding", "gzip")
    resp = urlopen(req)
    myzip = ZipFile(BytesIO(resp.read()))
    print("downloaded file")

    with myzip.open("trips.txt") as f:
        trips_df = pd.read_csv(f, dtype={
            "route_id": str,
            "service_id": np.int64,
            "trip_id": np.int64,
            "direction_id": np.int64,
            "shape_id": np.int64,
            "trip_headsign": str,
        })
        trips_df = trips_df[["route_id", "trip_id", "shape_id", "direction_id"]]

    with myzip.open("routes.txt") as f:
        routes_df = pd.read_csv(f, dtype={
            "route_id": str,
            "agency_id": np.int64,
            "route_type": np.int64,
            "route_short_name": str,
            "route_long_name": str,
            "route_desc": str,
        })

    with myzip.open("agency.txt") as f:
        agency_df = pd.read_csv(f, dtype={
            "agency_id": np.int64,
            "agency_name": str,
            "agency_url": str,
            "agency_timezone": str,
            "agency_lang": str,
            "agency_fare_url": str,
        })

    with myzip.open("stop_times.txt") as f:
        trip_headsign_df = pd.read_csv(f, dtype={
            "trip_id": np.int64,
            "stop_id": np.int64,
            "stop_headsign": str,
        }).drop_duplicates("trip_id", keep="first")
        trip_headsign_df.rename(columns={"stop_headsign": "trip_headsign"}, inplace=True)
    
    aggregated_df = trips_df \
        .merge(trip_headsign_df, on="trip_id", how="outer") \
        .merge(routes_df, on="route_id", how="left") \
        .merge(agency_df, on="agency_id", how="left")

    final_df = aggregated_df[["trip_id", "agency_name", "route_short_name", "route_long_name", "route_type", "trip_headsign", "shape_id", "direction_id", "route_id"]]

    storage_client = storage.Client()
    bucket = storage_client.bucket("gtfs_static")
    blob = bucket.blob("sweden_aggregated_metadata.csv")

    contents = final_df.to_csv(index=False, encoding="utf-8")
    blob.upload_from_string(bytes(contents, "utf-8"))

    print("======================================")
    print("refreshed static GTFS Sweden aggregate")
    print("======================================")
