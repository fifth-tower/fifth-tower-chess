## 象棋游戏网页

主要包括：象棋对战、聊天、邀请好友功能
技术架构：daisyUI+leptos+bevy+matchbox

    - 网页布局使用daisyUI+tailwind实现
    - 对战逻辑、音频、图片使用bevy实现,通讯使用webrtc协议,
    - 聊天通讯使用webrtc协议
    - webrtc协议由matchbox实现

[`demo`](http://chess.5-tower.online/), 可打开两个页面进入对战模式，可见全部功能

## Developing

```sh
trunk serve --port 3000 --open
```

will open your app in your default browser at `http://localhost:3000`.

## Deploying

To build

```sh
trunk build --release
```

This will output the files necessary to run your app into the `dist` folder; you can then use any static site host to serve these files.
