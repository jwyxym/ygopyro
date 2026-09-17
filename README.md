# ygopyro

`ygopyro` 是一个基于 Rust 和 PyO3 的 Python 扩展，用于启动 YGOPro 本地对战服务器。

项目使用 Tokio 异步运行时，服务器入口可以在 Python 的 `asyncio` 程序中调用。

## 环境要求

- Python 3.13 或更高版本
- Windows、Linux 或 macOS
- 从源码构建时需要 Rust 工具链

## 安装

从 PyPI 安装：

```bash
pip install ygopyro
```

使用 `uv` 安装：

```bash
uv add ygopyro
```

## 使用

`start_server` 是一个异步函数。它会启动一个后台服务器并返回唯一的服务器 ID；停止服务器时将该 ID 传给 `stop_server`：

```python
import asyncio

from ygopyro import start_server, stop_server


async def main() -> None:
    server_id = await start_server(
        port=7911,
        lflist=0,
        rule=0,
        mode=0,
        replay_mode=0,
        duel_rule=False,
        no_check_deck=False,
        no_shuffle_deck=False,
        start_lp=8000,
        start_hand=5,
        draw_count=1,
        time_limit=180,
    )

    # 运行一段时间后停止指定服务器
    await asyncio.sleep(10)
    stop_server(server_id)


if __name__ == "__main__":
    asyncio.run(main())
```
