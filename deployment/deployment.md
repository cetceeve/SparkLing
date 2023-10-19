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

# Deploying a secret service into the cluster

This is an abosolutly crutial step on first deployment!
```
kubectl create secret generic api-key-secret --from-literal=TRAFIKLAB_GTFS_RT_KEY='{paste here}' --from-literal=TRAFIKLAB_GTFS_STATIC_KEY='{paste here}'
```

# Remove deployment
```
kubectl --namespace default scale deployment {my-deployment} --replicas 0
```