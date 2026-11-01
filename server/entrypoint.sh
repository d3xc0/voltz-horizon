#!/bin/bash
# Pelican's yolk contract: run the egg's startup line with {{VAR}} expanded.
cd /home/container || exit 1
INTERNAL_IP=$(ip route get 1 | awk '{print $(NF-2);exit}')
export INTERNAL_IP
MODIFIED_STARTUP=$(echo -e "${STARTUP}" | sed -e 's/{{/${/g' -e 's/}}/}/g')
echo ":/home/container$ ${MODIFIED_STARTUP}"
eval "${MODIFIED_STARTUP}"
