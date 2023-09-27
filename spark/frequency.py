from pyspark.sql import SparkSession #, StructType, StructField, StringType, DoubleType
from pyspark.sql.window import Window
import json

# input_schema = StructType([
#     StructField("vehicle_id", StringType),
#     StructField("position", StructType([
#         StructField("lat", DoubleType),
#         StructField("long", DoubleType),
#         StructField("bearing", DoubleType),
#         StructField("speed", DoubleType),
#     ])),
# ])

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

# json_df = input_df.select(
#     # col("key").alias("timestamp"),
#     from_json(col("value").cast("string"), input_schema).alias("value")
# )

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
#     .rowsBetween(-2, 0)
#     # .orderBy("timestamp")

# windowed_df = row_df \
#     .withColumn("frequency", )

# writing a data stream to kafka
ds = input_df \
    .selectExpr("CAST(key AS STRING)", "CAST(value AS STRING)") \
    .writeStream \
    .format("kafka") \
    .option("checkPointLocation", "/tmp/spark/frequency/checkpoint") \
    .option("kafka.bootstrap.servers", "kafka:9092") \
    .option("topic", "realtime2") \
    .start() \
    .awaitTermination()
