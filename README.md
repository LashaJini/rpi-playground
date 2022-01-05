```bash
# Build docker image and cargo
./docker.sh --build

# Deploy binary and create service
./docker.sh --run --create-service

# Deploy binary and don't create service
./docker.sh --run --no-service

# Build & deploy
./docker.sh --all --create-service
./docker.sh --all --no-service
```
