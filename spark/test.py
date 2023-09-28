from pyspark.sql import SparkSession
import pyspark.sql.functions as F

from urllib.request import urlopen, Request
from io import BytesIO
from zipfile import ZipFile
import pandas as pd
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
    req = Request(f"https://opendata.samtrafiken.se/gtfs-sweden/sweden.zip?key={TRAFIKLAB_GTFS_STATIC_KEY}")
    req.add_header("accept", "application/octet-stream")
    req.add_header("accept-encoding", "gzip")
    req.add_header("if-none-match", "bfc13a64729c4290ef5b2c2730249c88ca92d82d")
    # req.add_header("if-modified-since", "Mon, 13 Jul 2020 04:24:36 GMT")
    resp = urlopen(req)
    myzip = ZipFile(BytesIO(resp.read()))
    # myzip = ZipFile("../static.gtfs.gz")
    print("downloaded file")

    with myzip.open("trips.txt") as f:
        df = pd.read_csv(f)
        trips_df = spark.createDataFrame(df)

    with myzip.open("routes.txt") as f:
        df = pd.read_csv(f)
        routes_df = spark.createDataFrame(df)

    with myzip.open("agency.txt") as f:
        df = pd.read_csv(f)
        agency_df = spark.createDataFrame(df)

    with myzip.open("routes.txt") as f:
        df = pd.read_csv(f)
        routes_df = spark.createDataFrame(df)

    meta_df = trips_df \
        .join(routes_df, on="route_id", how="inner") \
        .join(agency_df, on="agency_id", how="inner") \
        .select(
            F.col("trip_id"),
            F.col("agency_name"),
            F.col("route_short_name"),
            F.col("route_long_name"),
        )

    meta_df.printSchema()
    meta_df.show(10)
    meta_df.persist()

refreshStaticData()
