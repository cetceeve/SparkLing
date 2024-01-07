#!/usr/bin/env bash
if [ -z "$1" ]
  then
    echo "Missing target platform parameter"
fi

docker buildx build --platform=$1 -t jonathanarns/sparkling-event-engine ./event-engine
docker buildx build --platform=$1 -t jonathanarns/sparkling-app ./website
docker buildx build --platform=$1 -t fabianzeiher/sparkling-inference-pipeline ./inference-pipeline
docker buildx build --platform=$1 -t fabianzeiher/sparkling-feature-uploader ./feature-uploader

docker push jonathanarns/sparkling-event-engine:latest
docker push jonathanarns/sparkling-app:latest
docker push fabianzeiher/sparkling-inference-pipeline:latest
docker push fabianzeiher/sparkling-feature-uploader:latest
