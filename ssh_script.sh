#!/bin/bash

# This scripts runs when we ssh into raspberry pi

set -o xtrace -o nounset -o errexit -o pipefail

SERVICE_NAME="${1-}"
SERVICE_PATH=/etc/systemd/system/${SERVICE_NAME}
SERVICE_ACTION="${2-}"
SUCCESS=0

create_service () {
  echo -e "\n[***] Service Does Not Exist...Creating...\n"

  sudo cp /tmp/${SERVICE_NAME} ${SERVICE_PATH}

  sudo systemctl daemon-reload && \
    sudo systemctl enable ${SERVICE_NAME} && \
    sudo systemctl start ${SERVICE_NAME} && \
    SUCCESS=1

  if [[ "${SUCCESS}" -eq 0 ]]; then
    echo -e "\n[!!!] Failed service creation...Removing...\n"
    sudo rm ${SERVICE_PATH}
  fi
}

if [[ "$SERVICE_ACTION" = "--no-service" ]]; then
  echo -e "\n[***] Skipping service creation...\n"
elif [[ "$SERVICE_ACTION" = "--create-service" ]]; then
  if [ ! -f ${SERVICE_PATH} ];then
    create_service
  else
    echo -e "\n[***] Service Exists...Restarting...\n"

    sudo systemctl restart ${SERVICE_NAME}
  fi
else
  echo "[!!!] Error: Invalid Service Action flag passed"
  exit 1
fi
