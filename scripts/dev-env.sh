#!/bin/sh

SESSIONNAME="sauna-dev-env"

tmux has-session -t $SESSIONNAME

if [ $? != 0 ]
  then
    tmux new -s $SESSIONNAME -d \; \
        send-keys "cd \"$(dirname $0)/../\"" C-m \; \
        send-keys "vim" C-m \; \
        split-window -h -p 10 \; \
        send-keys 'cargo watch -x run' C-m \; \
        split-window -v \; \
        select-pane -t 0
fi

tmux attach -t $SESSIONNAME
