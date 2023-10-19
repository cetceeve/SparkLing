from pyspark.sql import SparkSession
from pyspark.sql.types import StructType, StructField, StringType, DoubleType, LongType
from pyspark.sql.window import Window
from pyspark import SparkFiles
import pyspark.sql.functions as F

spark = SparkSession \
    .builder \
    .config("spark.files.overwrite", "true") \
    .master("k8s://sparkling:7078") \
    .appName("SparkLingMetadataJoin") \
    .getOrCreate()

# csv file containing the aggregates for gtfs static sweden hosted in storage bucket
spark.sparkContext.addFile('https://storage.googleapis.com/gtfs_static/sweden_aggregated_metadata.csv')

STATIC_SCHEMA = StructType([
    StructField("trip_id", LongType(), True),
    StructField("agency_name", StringType(), True),
    StructField("route_short_name", StringType(), True),
    StructField("route_long_name", StringType(), True),
    StructField("route_type", LongType(), True),
    StructField("trip_headsign", StringType(), True),
])

RT_SCHEMA = StructType() \
    .add(StructField("vehicle_id", StringType())) \
    .add(StructField("trip_id", LongType())) \
    .add(StructField("latitude", DoubleType())) \
    .add(StructField("longitude", DoubleType())) \
    .add(StructField("bearing", DoubleType())) \
    .add(StructField("speed", DoubleType())) \

def run_streaming_query():
    """This refreshes the static dataframe with the latest static GTFS dataset"""
    static_df = spark.read \
        .schema(STATIC_SCHEMA) \
        .csv(SparkFiles.get("sweden_aggregated_metadata.csv"), header=False, sep = ",")
    
    static_df.show()
    print("=======================================")
    print("refreshed static GTFS Sweden dataframe")
    print("=======================================")

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

    join_df = streaming_df.join(static_df, on="trip_id", how="left_outer")
    streaming_query = join_df \
        .select(
            "key",
            "timestamp",
            F.to_json(F.struct(
                "vehicle_id", "trip_id", "latitude", "longitude",
                "agency_name", "route_short_name", "route_long_name",
                "trip_headsign", "route_type",
            )).alias("value")
        ) \
        .writeStream \
        .format("kafka") \
        .option("checkPointLocation", "/tmp/spark/frequency/checkpoint") \
        .option("kafka.bootstrap.servers", "kafka:9092") \
        .option("topic", "realtime_with_metadata") \
        .start()
    return streaming_query

# periodically re-start streaming query to re-load static data
while True:
    print("START: starting execution...")
    streaming_query = run_streaming_query()
    streaming_query.awaitTermination(60 * 60 * 24)
    streaming_query.stop()
    print("DEBUG: restarting streaming query after reloading static dataset")
