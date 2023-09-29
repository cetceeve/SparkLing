from pyspark.sql import SparkSession
from pyspark.sql.types import StructType, StructField, StringType, DoubleType, LongType
from pyspark.sql.window import Window
from pyspark import SparkFiles
import pyspark.sql.functions as F

from io import BytesIO
from zipfile import ZipFile
from urllib.request import urlopen, Request
from time import sleep
import pandas as pd
import os


TRAFIKLAB_GTFS_STATIC_KEY = os.getenv("TRAFIKLAB_GTFS_STATIC_KEY")

spark = SparkSession \
    .builder \
    .master("spark://spark:7077") \
    .appName("SparkLingMetadataJoin") \
    .getOrCreate()

# Create the static Dataframe initially with dummy data
SCHEMA = StructType([
    StructField("trip_id", LongType(), True),
    StructField("agency_name", StringType(), True),
    StructField("route_short_name", StringType(), True),
    StructField("route_long_name", StringType(), True),
])
STATIC_DF = spark.createDataFrame([(0, "ABC", "18", None),], schema=SCHEMA)
STATIC_DF = STATIC_DF.persist()

# Define a method that refreshes the static Dataframe
def refreshStaticData():
    """This function refreshes the static dataframe with the latest static GTFS dataset"""
    global STATIC_DF
    # global TRAFIKLAB_GTFS_STATIC_KEY
    # req = Request(f"https://opendata.samtrafiken.se/gtfs-sweden/sweden.zip?key={TRAFIKLAB_GTFS_STATIC_KEY}")
    # req.add_header("accept", "application/octet-stream")
    # req.add_header("accept-encoding", "gzip")
    # req.add_header("if-none-match", "bfc13a64729c4290ef5b2c2730249c88ca92d82d")
    # # req.add_header("if-modified-since", "Mon, 13 Jul 2020 04:24:36 GMT")
    # resp = urlopen(req)
    # myzip = ZipFile(BytesIO(resp.read()))
    myzip = ZipFile(SparkFiles.get("static.gtfs.gz"))
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

    STATIC_DF = STATIC_DF.unpersist(True)
    STATIC_DF = trips_df \
        .join(routes_df, on="route_id", how="inner") \
        .join(agency_df, on="agency_id", how="inner") \
        .select(
            F.col("trip_id"),
            F.col("agency_name"),
            F.col("route_short_name"),
            F.col("route_long_name"),
        )
    STATIC_DF = STATIC_DF.persist()
    STATIC_DF.show()
    print("=======================================")
    print("refreshed static GTFS Sweden dataframe")
    print("=======================================")

# Read actual streaming data and perform join operation with static Dataframe
# As an example I used Kafka as a streaming source
RT_SCHEMA = StructType() \
    .add(StructField("vehicle_id", StringType())) \
    .add(StructField("trip_id", StringType())) \
    .add(StructField("latitude", DoubleType())) \
    .add(StructField("longitude", DoubleType())) \
    .add(StructField("bearing", DoubleType())) \
    .add(StructField("speed", DoubleType())) \

def run_streaming_query():
    """This function defines our realtime streaming data pipeline and executes it"""
    streaming_df = spark \
        .readStream \
        .format("kafka") \
        .option("kafka.bootstrap.servers", "kafka:9092") \
        .option("subscribe", "realtime") \
        .load() \
        .withColumn("value", F.col("value").cast("STRING")) \
        .withColumn("value", F.regexp_replace("value", "\"", "")) \
        .withColumn("csv", F.from_csv(F.col("value"), RT_SCHEMA.simpleString())) \
        .select("key", "csv.*", "timestamp")

    join_df = streaming_df.join(STATIC_DF, on="trip_id", how="left_outer")
    streaming_query = join_df \
        .select(
            "key",
            "timestamp",
            F.to_json(F.struct("vehicle_id", "trip_id", "latitude", "longitude",
                            "bearing", "speed", "agency_name",
                            "route_short_name", "route_long_name"
                            )).alias("value")
        ) \
        .writeStream \
        .format("kafka") \
        .option("checkPointLocation", "/tmp/spark/frequency/checkpoint") \
        .option("kafka.bootstrap.servers", "kafka:9092") \
        .option("topic", "realtime_with_metadata") \
        .start()
    return streaming_query

# call refresh once on startup to load the static data
# then periodically re-start streaming query to re-load static data
refreshStaticData()
while True:
    streaming_query = run_streaming_query()
    sleep(30)
    refreshStaticData()
    streaming_query.stop()
    print("DEBUG: restarting streaming query after reloading static dataset")
