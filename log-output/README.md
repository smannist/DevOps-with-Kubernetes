# Log-output

## Running locally

```
cargo run
```

## Running with Docker

```
docker build -t smannist/log-output:1.0 .
docker run smannist/log-output:1.0
```

## Deploying to Kubernetes

```
kubectl create deployment log-output --image=smannist/log-output:1.0
kubectl logs -f deployment/log-output
```
