from pyspark.sql import SparkSession
import pyspark.sql.functions as F

from urllib.request import urlopen, Request
from io import BytesIO
from zipfile import ZipFile
import pandas as pd
import numpy as np
import os


TRAFIKLAB_GTFS_STATIC_KEY = os.getenv("TRAFIKLAB_GTFS_STATIC_KEY")

spark = SparkSession \
    .builder \
    .master("local") \
    .appName("SparkLingMetadataJoin") \
    .getOrCreate()

# # Load the staticDataframe initially and keep as `var`
# staticDf = spark.read.format("delta").load(deltaPath)
# staticDf.persist()

def refreshStaticData():
    # req = Request(f"https://opendata.samtrafiken.se/gtfs-sweden/sweden.zip?key={TRAFIKLAB_GTFS_STATIC_KEY}")
    # req.add_header("accept", "application/octet-stream")
    # req.add_header("accept-encoding", "gzip")
    # req.add_header("if-none-match", "bfc13a64729c4290ef5b2c2730249c88ca92d82d")
    # req.add_header("if-modified-since", "Mon, 13 Jul 2020 04:24:36 GMT")
    # resp = urlopen(req)
    # myzip = ZipFile(BytesIO(resp.read()))
    myzip = ZipFile("../static.gtfs.gz")
    print("downloaded file")

    with myzip.open("trips.txt") as f:
        df = pd.read_csv(f, dtype={
            "route_id": str,
            "service_id": np.int64,
            "trip_id": np.int64,
            "direction_id": np.int64,
            "shape_id": np.int64,
            "trip_headsign": str,
        })
        df[["trip_headsign"]] = df[["trip_headsign"]].fillna("")
        trips_df = spark.createDataFrame(df[["route_id", "trip_id"]])

    with myzip.open("routes.txt") as f:
        df = pd.read_csv(f, dtype={
            "route_id": str,
            "agency_id": np.int64,
            "route_type": np.int64,
            "route_short_name": str,
            "route_long_name": str,
            "route_desc": str,
        })
        df[["route_short_name", "route_long_name", "route_desc"]] = df[["route_short_name", "route_long_name", "route_desc"]].fillna("")
        routes_df = spark.createDataFrame(df)

    with myzip.open("agency.txt") as f:
        df = pd.read_csv(f, dtype={
            "agency_id": np.int64,
            "agency_name": str,
            "agency_url": str,
            "agency_timezone": str,
            "agency_lang": str,
            "agency_fare_url": str,
        })
        df[["agency_name", "agency_url", "agency_timezone", "agency_lang", "agency_fare_url"]] = df[["agency_name", "agency_url", "agency_timezone", "agency_lang", "agency_fare_url"]].fillna("")
        agency_df = spark.createDataFrame(df[["agency_id", "agency_name"]])

    with myzip.open("stop_times.txt") as f:
        df = pd.read_csv(f, dtype={
            "trip_id": np.int64,
            "stop_id": np.int64,
            "stop_headsign": str,
        })
        df = df.drop_duplicates("trip_id", keep="first")
        df[["trip_headsign"]] = df[["stop_headsign"]].fillna("")
        trip_headsign_df = spark.createDataFrame(df[["trip_id", "headsign"]]) \

    meta_df = trips_df \
        .join(trip_headsign_df, on="trip_id", how="left_outer") \
        .join(routes_df, on="route_id", how="inner") \
        .join(agency_df, on="agency_id", how="inner") \
        .select(
            F.col("trip_id"),
            F.col("agency_name"),
            F.col("route_short_name"),
            F.col("route_long_name"),
            F.col("route_type"),
            F.col("trip_headsign"),
        )

    meta_df.printSchema()
    meta_df.show(20)
    meta_df.persist()

refreshStaticData()
