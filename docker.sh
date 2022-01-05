#!/bin/bash

# set -o xtrace -o nounset -o errexit -o pipefail

CONTAINER_NAME=rpi-playground
ACTION="${1-}"
SERVICE_ACTION="${2-}"

# logs out key :/
docker_build () {
  docker build \
    --build-arg ssh_prv_key="$(cat ~/.ssh/rpi)" \
    --build-arg ssh_pub_key="$(cat ~/.ssh/rpi.pub)" \
    -t ${CONTAINER_NAME} .
}

docker_run () {
  docker run -e SERVICE_ACTION=${SERVICE_ACTION} \
    -it --rm --name ${CONTAINER_NAME}-docker \
    ${CONTAINER_NAME}
}

usage () {
  echo -e "Usage:\n"
  echo "./docker.sh <ACTION> [SERVICE_ACTION]"
  echo -e "\tACTION\t: --build, --run, --all"
  echo -e "\tSERVICE_ACTION\t: --create-service, --no-service"
}

valid_service_action () {
  if [ ! "$SERVICE_ACTION" = "--create-service" ] &&
    [ ! "$SERVICE_ACTION" = "--no-service" ]; then
    echo "[!!!] Error: Invalid Service Action flag passed"
    usage
    exit 1
  fi
}


if [[ "$ACTION" = "--build" ]]; then
  docker_build
elif [[ "$ACTION" = "--run" ]]; then
  valid_service_action && docker_run
elif [[ "$ACTION" = "--all" ]]; then
  valid_service_action && docker_build && docker_run
else
  echo "[!!!] Error: No Action flag passed"
  usage
  exit 1
fi
