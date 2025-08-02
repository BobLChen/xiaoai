# 构建xiaoai程序

**重要**:构建在windows上面完成，mac下我暂时没搞定。

1、下载安装rust
```
# 下载 rustup-init.exe（64位）
https://www.rust-lang.org/zh-CN/tools/install

# 然后跟着提示安装
```

2、安装corss交叉编译
```
cargo install cross --git https://github.com/cross-rs/cross
```

3、安装docker
```
# 根据提示安装就行，后面会有wls2的安装提示
https://www.docker.com/
```

4、交叉编译
```
cross build --target aarch64-unknown-linux-gnu
```

# 部署xiaoai程序

1、本地启动一个简易python http服务，需要能够覆盖**init.sh** **xiaoai**
```
# 记下地址
python -m http.server 8001
```

2、打开小爱ssh
```
# 192.168.2.197改成你的小爱地址
ssh -o HostKeyAlgorithms=+ssh-rsa root@192.168.2.197

# 输入密码：open-xiaoai
```

3、创建目录
```
mkdir -p /data/xiaoai
cd /data/xiaoai/
```

4、设置server地址
```
echo 'ws://192.168.2.83:8092' > /data/xiaoai/server.txt
```

5、下载程序到小爱
```
# 本地python简易http服务覆盖的地址
curl -L -# -o xiaoai http://192.168.2.83:8001/xiaoai/target/armv7-unknown-linux-gnueabihf/release/xiaoai

# 启动程序
curl -sSfL http://192.168.2.83:8001/xiaoai/init.sh | sh
```

6、设置为自动启动
```
# 下载sh到/data/init.sh，刷机后会自动调用该脚本
curl -L -o /data/init.sh http://192.168.2.83:8001/xiaoai/init.sh

# 给予执行权限
chmod +x /data/init.sh

# 重启小爱音箱
reboot
```

# server

1、安装python3
```
https://www.python.org/downloads/
```

2、安装依赖库
```
pip install websockets
```

3、启动
```
python ./server/app.py
```