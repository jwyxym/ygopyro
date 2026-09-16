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

`start_server` 是一个异步函数，需要使用 `await` 调用：

```python
import asyncio

from ygopyro import start_server


async def main() -> None:
    await start_server(
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


if __name__ == "__main__":
    asyncio.run(main())
```

服务器会持续运行，结束程序时可以使用 `Ctrl+C`。