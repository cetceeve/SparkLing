from pyspark.sql import SparkSession
from pyspark.sql.types import StructType, StructField, StringType, DoubleType
from pyspark.sql.window import Window
import pyspark.sql.functions as F
import json

input_schema = StructType([
    StructField("vehicle_id", StringType()),
    StructField("position", StructType([
        StructField("lat", DoubleType()),
        StructField("long", DoubleType()),
        StructField("bearing", DoubleType()),
        StructField("speed", DoubleType()),
    ])),
])

spark = SparkSession \
    .builder \
    .master("spark://spark:7077") \
    .appName("SparkLingFrequency") \
    .getOrCreate()

# loading a data stream from kafka
input_df = spark \
    .readStream \
    .format("kafka") \
    .option("kafka.bootstrap.servers", "kafka:9092") \
    .option("subscribe", "realtime") \
    .load()

json_df = input_df.select(
    F.col("timestamp"),
    F.col("key"),
    F.from_json(F.col("value").cast("string"), input_schema).alias("value")
)

# row_df = json_df.selectExpr(
#     # "timestamp",
#     "value.vehicle_id as vehicle_id",
#     "value.position.lat as lat",
#     "value.position.long as long",
#     "value.position.bearing as bearing",
#     "value.position.speed as speed",
# )

# window_spec = Window \
#     .partitionBy("vehicle_id") \
#     .rowsBetween(-2, 0) \
#     .orderBy("timestamp")

#     # .withColumn("vehicle_id", json_df.selectExpr("value.vehicle_id as vehicle_id")) \
# prev_time_df = json_df \
#     .withColumn(
#         "prev_timestamp",
#         F.lag(json_df["timestamp"]).over(Window.partitionBy("value.vehicle_id").orderBy("timestamp"))
#     )
# time_delta_df = prev_time_df \
#     .withColumn("time_delta", prev_time_df["timestamp"] - prev_time_df["prev_timestamp"])

# writing a data stream to kafka
output_df = json_df \
    .select(
        F.col("key").cast("string").alias("key"),
        F.to_json(F.col("value")).cast("string").alias("value"),
    )
ds = output_df \
    .writeStream \
    .format("kafka") \
    .option("checkPointLocation", "/tmp/spark/frequency/checkpoint") \
    .option("kafka.bootstrap.servers", "kafka:9092") \
    .option("topic", "realtime2") \
    .start() \
    .awaitTermination()
