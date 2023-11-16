#!/usr/bin/env bash
if [ -z "$1" ]
  then
    echo "Missing target platform parameter"
fi

docker buildx build --platform=$1 -t jonathanarns/sparkling-event-engine ./event-engine
docker buildx build --platform=$1 -t jonathanarns/sparkling-app ./website

docker push $2/sparkling-event-engine:latest
docker push $2/sparkling-app:latest
