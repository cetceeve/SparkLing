#!/usr/bin/env bash

docker buildx build --platform=linux/amd64 -t $1/sparkling-api-client ./api_client
docker buildx build --platform=linux/amd64 -t $1/sparkling-app ./website
docker buildx build --platform=linux/amd64 -t $1/sparkling-spark -f ./spark/Dockerfile.prod ./spark

docker push $1/sparkling-api-client:latest
docker push $1/sparkling-app:latest
docker push $1/sparkling-spark:latest
