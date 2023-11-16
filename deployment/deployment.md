# Deployment

# Building images

Make sure to build for amd64 architecture using docker buildx:

```
docker buildx build --platform=linux/amd64 -t test-image ./test
```

You can use the `build.sh` script in the project root.
You can specify which architecture you need and it automatically pushed to dockerhub.
```
./build.sh linux/amd64
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

## 2. Deploy redis
First, we need our message queue.
```
kubectl apply -f deployment/redis.yml
```
The command creates
- redis deployment
- sparkling-redis service

## 3. Deploy event-engine
Next we deploy our event-engine that handles the processing of events.
```
kubectl apply -f deployment/event-engine.yml
```
The command creates
- event-engine deployment

## 4. Deploy web application
```
kubectl apply -f deployment/web-app.yml
```
The command creates
- web-app deployment
- web-app load balancer service (with our static IP)

## 9. Deploy vertical autoscalers

> !Vertical autoscaling is not possible for JVM tasks!
```
kubectl apply -f deployment/vertical-autoscalers.yml
```
The command creates
- event-engine vertical autoscaler
- web-app vertical autoscaler

# Redeployment

Normally you should only need to repeat from step 2.

# Remove deployment
```
kubectl --namespace default scale deployment {my-deployment} --replicas 0
```
