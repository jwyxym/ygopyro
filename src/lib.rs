use pyo3::prelude::*;
use std::{
    sync::{Mutex, OnceLock, atomic::AtomicU64},
    collections::HashMap
};
type ShutdownSender = tokio::sync::oneshot::Sender<()>;

static NEXT_SERVER_ID: AtomicU64 = AtomicU64::new(1);
static SERVER_CONTROL: OnceLock<Mutex<HashMap<u64, ShutdownSender>>> = OnceLock::new();

#[pymodule]
mod _core {
    use super::{NEXT_SERVER_ID, SERVER_CONTROL};
    use pyo3::prelude::*;
    use pyo3_async_runtimes::tokio::future_into_py;
    use std::{
        sync::{Mutex, atomic::Ordering},
        collections::HashMap
    };
    use tokio::{net::TcpListener, select, sync::oneshot};
    use ygopro::{
        DuelHost,
        cli::{build_duel_host, start_local_server_with_listener},
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
    ) -> PyResult<Bound<'_, PyAny>> {
        let seeds: Vec<[u32; SEED_COUNT]> = Vec::new();
        let replay_mode = ReplayMode::from_bits_retain(replay_mode);
        let duel_rule = if duel_rule {
            MasterRule::MasterRuleNew
        } else {
            MasterRule::MasterRule2020
        };
        let mode = Mode::try_from(mode).unwrap_or(Mode::Single);
        let host_info = HostInfo {
            lflist,
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
            let server_id = NEXT_SERVER_ID.fetch_add(1, Ordering::Relaxed);
            let (shutdown_sender, shutdown_receiver) = oneshot::channel();

            SERVER_CONTROL
                .get_or_init(|| Mutex::new(HashMap::new()))
                .lock()
                .map_err(|_| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                        "server control lock is poisoned",
                    )
                })?
                .insert(server_id, shutdown_sender);

            tokio::spawn(async move {
                ygopro::init();

                let Ok(listener) = TcpListener::bind(("0.0.0.0", port)).await else {
                    remove_server(server_id);
                    return;
                };
                let duel: DuelHost = build_duel_host(host_info, replay_mode, seeds);

                select! {
                    _ = shutdown_receiver => {}
                    _ = start_local_server_with_listener(listener, duel) => {}
                }

                remove_server(server_id);
            });

            Ok(server_id)
        })
    }

    #[pyfunction]
    fn stop_server(server_id: u64) -> PyResult<()> {
        let shutdown_sender = SERVER_CONTROL
            .get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .map_err(|_| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "server control lock is poisoned",
                )
            })?
            .remove(&server_id);

        match shutdown_sender {
            Some(sender) => {
                let _ = sender.send(());
                Ok(())
            }
            None => Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("unknown or already stopped server id: {server_id}"),
            )),
        }
    }

    fn remove_server(server_id: u64) {
        if let Ok(mut servers) = SERVER_CONTROL
            .get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
        {
            servers.remove(&server_id);
        }
    }
}
