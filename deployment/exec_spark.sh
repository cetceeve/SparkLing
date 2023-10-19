#!/usr/bin/env bash


~/Code/spark-3.5.0-bin-hadoop3/bin/spark-submit \
  --master k8s://https://127.0.0.1:51317 \
  --deploy-mode cluster \
  --name sparkling \
  --class org.apache.spark.examples.SparkPi \
  --conf spark.kubernetes.container.image=docker.io/fabianzeiher/sparkling-py-spark:latest \
  --conf spark.kubernetes.context=minikube \
  --conf spark.kubernetes.driver.pod.name=spark \
  --conf spark.kubernetes.namespace=spark \
  --conf spark.kubernetes.authenticate.driver.serviceAccountName=spark \
  --conf spark.kubernetes.file.upload.path='local:///opt/spark/jars' \
  --conf spark.dynamicAllocation.enabled=false \
  --conf spark.executor.instances=1 \
  --conf spark.kubernetes.container.image.pullPolicy=Always \
  --conf spark.ui.enabled=false \
  --verbose \
  local:///app/rt_stream_only.py

