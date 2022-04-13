#!/usr/bin/env bash

set -e
. _common.bash

WORKSPACE=${WORKSPACE:="empty"}

init

sanity_check "${WORKSPACE}"

create_repo "${WORKSPACE}"

archive "${WORKSPACE}"

cleanup "${WORKSPACE}"
