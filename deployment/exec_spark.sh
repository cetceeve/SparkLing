#!/usr/bin/env bash


~/Code/spark-3.5.0-bin-hadoop3/bin/spark-submit \
  --master k8s://https://34.88.64.173 \
  --deploy-mode cluster \
  --name sparkling \
  --class org.apache.spark.examples.SparkPi \
  --jars local:///opt/spark/jars/gcs-connector-hadoop2-latest.jar \
  --packages org.apache.spark:spark-sql-kafka-0-10_2.12:3.4.0 \
  --conf spark.jars.ivy=/tmp/.ivy \
  --conf spark.kubernetes.container.image=docker.io/fabianzeiher/sparkling-py-spark:latest \
  --conf spark.kubernetes.driver.pod.name=spark \
  --conf spark.kubernetes.namespace=default \
  --conf spark.kubernetes.authenticate.driver.serviceAccountName=spark \
  --conf spark.kubernetes.file.upload.path='local:///opt/spark/jars' \
  --conf spark.hadoop.fs.AbstractFileSystem.gs.impl=com.google.cloud.hadoop.fs.gcs.GoogleHadoopFS \
  --conf spark.dynamicAllocation.enabled=false \
  --conf spark.executor.instances=1 \
  --conf spark.kubernetes.container.image.pullPolicy=Always \
  --conf spark.ui.enabled=false \
  --verbose \
  local:///app/rt_stream_only.py


#--jars local:///opt/spark/jars/spark-sql-kafka-0-10_2.12-3.4.0.jar \