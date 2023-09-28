from pyspark.sql import SparkSession, Column
from pyspark.sql.types import StructType, StructField, StringType, DoubleType, IntegerType
from pyspark.sql.window import Window
import pyspark.sql.functions as F
import json

input_schema = StructType() \
    .add(StructField("vehicle_id", StringType())) \
    .add(StructField("trip_id", StringType())) \
    .add(StructField("latitude", DoubleType())) \
    .add(StructField("longitude", DoubleType())) \
    .add(StructField("bearing", DoubleType())) \
    .add(StructField("speed", DoubleType())) \


spark = SparkSession \
    .builder \
    .master("spark://spark:7077") \
    .appName("SparkLingDeserialize") \
    .getOrCreate()

# loading a data stream from kafka
df = spark \
    .readStream \
    .format("kafka") \
    .option("kafka.bootstrap.servers", "kafka:9092") \
    .option("subscribe", "realtime") \
    .load() \
    .withColumn("value", F.col("value").cast("STRING")) \
    .withColumn("value", F.regexp_replace("value", "\"", "")) \
    .withColumn("csv", F.from_csv(F.col("value"), input_schema.simpleString())) \
    .select("key", "csv.*", "timestamp")
    

df.writeStream.format("console").outputMode("append").start().awaitTermination()

# # writing a data stream to kafka
# ds = json_df \
#     .select(
#         F.col("key").cast("string"),
#         F.to_json(F.col("value")).cast("string"),
#     ) \
#     .writeStream \
#     .format("kafka") \
#     .option("checkPointLocation", "/tmp/spark/frequency/checkpoint") \
#     .option("kafka.bootstrap.servers", "kafka:9092") \
#     .option("topic", "realtime2") \
#     .start() \
#     .awaitTermination()
