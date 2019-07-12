#!/bin/sh

SESSIONNAME="sauna-dev-env"

tmux has-session -t $SESSIONNAME

if [ $? != 0 ]
  then
    tmux new -s $SESSIONNAME -d \; \
        send-keys "vim \"$(dirname $0)/../\"" C-m \; \
        split-window -h -p 25 \; \
        send-keys 'cargo watch -x run' C-m \; \
        split-window -v \; \
        select-pane -t 0
fi

tmux attach -t $SESSIONNAME
