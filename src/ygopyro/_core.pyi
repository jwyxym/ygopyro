from typing import Awaitable


def start_server(
    port: int,
    lflist: int,
    rule: int,
    mode: int,
    replay_mode: int,
    duel_rule: bool,
    no_check_deck: bool,
    no_shuffle_deck: bool,
    start_lp: int,
    start_hand: int,
    draw_count: int,
    time_limit: int,
) -> Awaitable[int]: ...


def stop_server(server_id: int) -> None: ...
