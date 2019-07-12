#!/bin/sh

SESSIONNAME="sauna-dev-env"

tmux has-session -t $SESSIONNAME

if [ $? != 1 ]
then
    tmux kill-session -t $SESSIONNAME
fi
