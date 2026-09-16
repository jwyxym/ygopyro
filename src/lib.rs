use pyo3::prelude::*;

#[pymodule]
mod _core {
    use pyo3::prelude::*;
    use pyo3_async_runtimes::tokio::future_into_py;
    use ygopro::{
        DuelHost,
        cli::{
            build_duel_host,
            start_local_server
        }
    };
    use ygopro_core_wrapper::random::SEED_COUNT;
    use ygopro_data::{
        constants::{MasterRule, Mode, Rule},
        data::ReplayMode,
        message::HostInfo,
    };

    #[pyfunction]
    fn start_server(
        py: Python<'_>,
        port: u16,
        lflist: u32,
        rule: u8,
        mode: u8,
        replay_mode: u32,
        duel_rule: bool,
        no_check_deck: bool,
        no_shuffle_deck: bool,
        start_lp: u32,
        start_hand: u8,
        draw_count: u8,
        time_limit: u16,
    )-> PyResult<Bound<'_, PyAny>> {
        let seeds: Vec<[u32; SEED_COUNT]> = Vec::new();
        let replay_mode: ReplayMode = ReplayMode::from_bits_retain(replay_mode);
        let duel_rule: MasterRule = if duel_rule {
            MasterRule::MasterRuleNew
        } else {
            MasterRule::MasterRule2020
        };
        let mode: Mode = Mode::try_from(mode).unwrap_or(Mode::Single);
        let host_info: HostInfo = HostInfo {
            lflist: lflist,
            rule: Rule::try_from(rule).unwrap_or(Rule::All),
            duel_rule,
            no_check_deck,
            no_shuffle_deck,
            start_lp,
            start_hand,
            draw_count,
            time_limit,
            mode,
        };
        future_into_py(py, async move {
            ygopro::init();
            let duel: DuelHost = build_duel_host(host_info, replay_mode, seeds);
            start_local_server(port, duel).await;
            Ok(())
        })
    }
}
