#!/bin/bash
SCRIPT_PATH=`dirname $0`
CMD_PATH=$SCRIPT_PATH/../target/debug
$CMD_PATH/freight_paths && $CMD_PATH/freight_configure
$CMD_PATH/freight_tidy
