# Deployment

# Building images

Make sure to build for amd64 architecture using docker buildx:

```
docker buildx build --platform=linux/amd64 -t test-image ./test
```

# Pushing images

## docker hub

I think now it is better to use dockerhub since it is free.

## cloud artifact registry
Read about pushing images to google cloud here: [artifact registry documentation](https://cloud.google.com/artifact-registry/docs/docker/pushing-and-pulling#pushing)

> Current recommendation: Download gCloud CLI and authenticate with docker using credential helper.

- Default region: `europe-north1-b`
- Registry URL: `europe-north1-docker.pkg.dev/tonal-vector-401814/sparklingimages`

use the following command
```
docker push europe-north1-docker.pkg.dev/tonal-vector-401814/sparklingimages/{image name}
```

# Initial Deployment

## 1. Deploy secret service into the cluster

This is an abosolutly crutial step on first deployment!
```
kubectl create secret generic api-key-secret --from-literal=TRAFIKLAB_GTFS_RT_KEY='{paste here}' --from-literal=TRAFIKLAB_GTFS_STATIC_KEY='{paste here}'
```

## 2. Install strimzi operator
This installs the kafka operator into the cluster

The command creates
-  strimzi-cluster-operator deployment

## 3. Create a spark service account and access rules
This service account will allow spark to create new pods for drivers and executors
```
kubectl apply -f deployment/spark-application-rbac.yml
```
This command creates
- spark service account and associated rules

## 4. Deploy kafka
First, we need our message queue. It takes quite a while to start.
```
kubectl apply -f deployment/strimzi-kafka.yml
```
The command creates
- zookeeper pods
- zookeeper services
- kafka broker pods
- kafka broker service
- kafka bootstrap service (we use this service name to connect from our clients)
- entity-operator deployment

## 5. Deploy kafka topics
```
kubectl apply -f deployment/kafka-topics
```
The command creates
- kafka topic realtime
- kafka topoc realtime_with_metadata

## 6. Deploy api-client
The spark job needs data to run, so we start the api-client next
```
kubectl apply -f deployment/api-client.yml
```
The command creates
- api-client deployment

## 7. Deploy spark
The spark startup needs some time
```
kubectl apply -f deployment/spark-submit.yml
```
The command creates
- spark-submit pod
- spark pod
- spark executor pod

Use the following commands to follow the startup process
```
kubectl logs -f spark-submit
kubectl logs -f spark
```

## 8. Deploy web application
```
kubectl apply -f deployment/web-app.yml
```
The command creates
- web-app deployment
- web-app load balancer service (we can use this service to reach our app from the interwebs)

## 9. Deploy vertical autoscalers

> !Vertical autoscaling is not possible for JVM tasks!
```
kubectl apply -f deployment/vertical-autoscalers.yml
```
The command creates
- api-client vertical autoscaler
- web-app vertical autoscaler
- spark-submit vertical autoscaler

# Redeployment

Normally you should only need to repeat steps 5 to 8.

# Remove deployment
```
kubectl --namespace default scale deployment {my-deployment} --replicas 0
```

Remove kafka deployment
```
kubectl -n default delete $(kubectl get strimzi -o name -n default)
```
