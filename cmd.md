# 部分可用指令

```
# 阻断方式让小爱说话
/usr/sbin/tts_play.sh '你好'

# 播放指定url的音频
ubus call mediaplayer player_play_url {\"url\":\"file:///usr/share/sound/shutdown.mp3\",\"type\":1}

# 让小爱说话
ubus call mibrain text_to_speech '{\"text\":\"你好\", \"save\":0}'

# 设置小爱音量
ubus -t 1 call mediaplayer player_set_volume {\"volume\":"30"}

# 唤醒小爱
ubus -t 1 call mediaplayer player_wakeup {\"action\":\"start\"}
ubus -t 1 call mediaplayer player_wakeup {\"action\":\"stop\"}

# 让小爱闭嘴
ubus call mediaplayer player_play_operation  "{\"action\":\"pause\"}"

# 获取播放状态
ubus call mediaplayer player_get_play_status
```