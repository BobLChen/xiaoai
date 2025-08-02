import asyncio
import websockets
import json
import uuid
import socket

from datetime import datetime

async def handle_client(websocket):

    # 为新客户端生成唯一ID
    client_id = str(uuid.uuid4())
    print(f"Client {client_id} connected from {websocket.remote_address}")
    
    # 小爱已连接消息
    # welcome_msg = {
    #     "type": "system",
    #     "message": f"Welcome! Your ID is {client_id}",
    #     "timestamp": datetime.now().isoformat()
    # }
    welcome_msg = "/usr/sbin/tts_play.sh '服务器连接成功'"
    await websocket.send(welcome_msg)
    
    try:
        async for message in websocket:
            try:
                data = json.loads(message)
                # 这里处理解析小爱的事件
                print(data)
            except json.JSONDecodeError:
                pass
    except websockets.exceptions.ConnectionClosedError:
        print(f"Client {client_id} disconnected unexpectedly")
    finally:
        pass

async def main(host, port):
    server = await websockets.serve(
        handle_client,                  # 处理客户端连接的协程
        host,                           # 监听地址
        port,                           # 监听端口
        ping_interval = 20,             # 每20秒发送一次ping
        ping_timeout = 60,              # 60秒无响应则断开
        max_size = 10 * 1024 * 1024     # 最大消息大小10MB
    )

    print(f"WebSocket server started at ws://{host}:{port}")
    print("Press Ctrl+C to stop the server")

    # 保持服务器运行
    await server.wait_closed()

if __name__ == "__main__":
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.connect(("8.8.8.8", 80))
    ip = s.getsockname()[0]
    s.close()

    asyncio.run(main(host=ip, port=8092))