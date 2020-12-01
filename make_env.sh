#!/bin/bash

set -eu
cat <<EOT > .env
UID=`id -u`
GID=`id -g`
EOT
