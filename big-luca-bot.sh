#!/bin/bash

cd "$(dirname "$0")" || exit 1

OP="$1"
APP_NAME="big-luca-bot"
export BIG_LUCA_PIDFILE="/var/run/big-luca-bot.pid"

if [ -z "$HOME" ]; then
  export HOME=/root
fi

CARGO=$(which cargo)
if [ -z "$CARGO" ]; then
  export CARGO_DIR="/$HOME/.cargo"
  [ -s "$CARGO_DIR/env" ] && \. "$CARGO_DIR/env"  # This loads nvm
fi

start() {
  ENV=".env"
  if [ -f ".env.override" ]; then
    ENV=".env.override"
  fi
  set -a
  . $ENV
  set +a

  touch "$DATABASE_URI"

  CMD="$(which $APP_NAME)"
  if [ -z "$CMD" ]; then
    CMD="$CARGO run -r"
  else
    CMD="$CMD"
  fi

  screen -S $APP_NAME -d -m $CMD

  return $?
}

stop() {
  PID=$(cat $PIDFILE)

  kill "$PID"

  return $?
}

case "$1" in

  "start")
    start
    ;;
  
  "stop")
    stop
    ;;
  
  *)
    "unknown operation $OP"
    exit 1
    ;;

esac