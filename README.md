# PastebinIt-rs

![](https://hitscounter.dev/api/hit?url=https%3A%2F%2Fgithub.com%2Frsbench%2Frsbench&label=&icon=github&color=%23160d27) ![](https://img.shields.io/crates/v/pastebinit-rs) ![](https://tokei.rs/b1/github/GenshinMinecraft/pastebinit-rs)

一个使用 Rust 编写的 [pastebinit](https://github.com/skorokithakis/pastebinit) 替代品，支持多种 Pastebin 服务

## 已支持的 Pastebin 服务

- Debian Pastebin: https://paste.debian.net (Default)
- Centos Pastebin: https://paste.centos.org
- Meson Pastebin: https://pb.meson.cc/
- Note LinkOf Pastebin: https://note.linkof.link/
- TextDb Pastebin: https://textdb.online/
- OpenStack Pastebin: https://paste.openstack.org/

## 安装

### 一键脚本

Install:

```bash
bash <(wget -qO- -o- https://ghfast.top/https://raw.githubusercontent.com/GenshinMinecraft/pastebinit-rs/refs/heads/main/install.sh)
```

Uninstall:

```bash
bash <(wget -qO- -o- https://ghfast.top/https://raw.githubusercontent.com/GenshinMinecraft/pastebinit-rs/refs/heads/main/install.sh) uninstall
```

### Cargo Install

若已拥有 Cargo 与 Rust 开发环境，可直接使用该安装方法

```bash
cargo install pastebinit-rs
```

## 使用方法

```
Just Paste It! A simple CLI tool to paste text to various pastebin services.

Usage:

Options:
  -s, --server <SERVER>  Select the pastebin provider [default: centos] [possible values: debian, centos, meson, link-of, text-db, open-stack]
  -f, --file <FILE>      Select file to upload (Ignored stdin) [default: ]
  -t, --title <TITLE>    Set pastebin title [default: ]
  -p, --private          Set pastebin visibility
  -r, --raw              Show RAW Text link
  -h, --help             Print help
  -V, --version          Print version
```

该软件有三种工作方式，分别为`管道符号输入`、`终端输入`和`文件输入`

- 管道符号输入
    ```bash
    cat ./test.txt | pastebinit-rs
    ```
- 终端输入
    ```bash
    pastebinit-rs
    Run on a terminal, try to type something and end by `EOF`!
    Hello, world! (text.txt 的文件内容)
    EOF
    ```
- 文件输入
    ```bash
    pastebinit-rs -f ./test.txt
    ```

上面的三种方式理论上是等价的，只要 text.txt 的内容相同，上传到 Pastebin 的内容也是相同的

你可以继续添加参数:

- `-s` 或 `--server`: 选择 Pastebin 服务，可选: `debian` / `centos`，默认值为 `centos`
- `-t` 或 `--title`: 设置 Pastebin 的标题，默认值为空，并不所有的 Pastebin 服务都支持
- `-p` 或 `--private`: 设置 Pastebin 的可见性，默认值为公开，并不是所有的 Pastebin 服务都支持
- `-r` 或 `--raw`: 显示 RAW Text 链接，默认值为 false

若成功上传，程序会直接输出 Pastebin 的链接，无任何附加内容

若失败则会输出错误信息，并以 `1` 的状态码退出

## LICENSE

本项目根据 WTFPL 许可证开源

```
        DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE 
                    Version 2, December 2004 

 Copyright (C) 2004 Sam Hocevar <sam@hocevar.net> 

 Everyone is permitted to copy and distribute verbatim or modified 
 copies of this license document, and changing it is allowed as long 
 as the name is changed. 

            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE 
   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION 

  0. You just DO WHAT THE FUCK YOU WANT TO.# pastebinit-rs
