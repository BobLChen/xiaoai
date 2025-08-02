#! /bin/sh

exec > /dev/null 2>&1

WORK_DIR="/data/xiaoai"
CLIENT_BIN="$WORK_DIR/xiaoai"
SERVER_ADDRESS="ws://127.0.0.1:8092"

if [ ! -d "$WORK_DIR" ]; then
    mkdir -p "$WORK_DIR"
fi

if [ ! -f "$CLIENT_BIN" ]; then
    echo "🔥 xiaoai程序不存在..."
    exit 1
fi

chmod +x "$CLIENT_BIN"

if [ -f "$WORK_DIR/server.txt" ]; then
    SERVER_ADDRESS=$(cat "$WORK_DIR/server.txt")
fi

echo "🔥 xiaoai程序..."
kill -9 `ps|grep "xiaoai/xiaoai"|grep -v grep|awk '{print $1}'` > /dev/null 2>&1 || true
"$CLIENT_BIN" "$SERVER_ADDRESS"