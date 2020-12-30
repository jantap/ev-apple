use super::{Dispatch, Senders, common::*};

pub trait Message {
    fn id(&self) -> u32;
}

pub struct EventsManager<T: Send + std::fmt::Debug + Clone + Message + 'static> {
    sx: Sender<T>,
    rx: Receiver<T>,
    sync_sx: cb_channel::Sender<T>,
    sync_rx: cb_channel::Receiver<T>,
    exit_message_id: u32,
}

impl<T: Send + std::fmt::Debug + Clone + Message + 'static> EventsManager<T> {
    pub async fn prepare(dispatch: &Dispatch<T>, exit_message_id: u32) -> Self {
        let (sx, rx) = channel(32);
        let (sync_sx, sync_rx) = cb_channel::unbounded();

        dispatch.register(sx.clone(), sync_sx.clone()).await;

        Self {
            sx,
            rx,
            sync_rx,
            sync_sx,
            exit_message_id,
        }
    }
}

pub async fn start<T: Send + std::fmt::Debug + Clone + Message + 'static>(
    manager: EventsManager<T>,
    senders: Senders<T>,
) {
    let sync_rx = manager.sync_rx;
    let sync_sx = manager.sync_sx;
    let mut rx = manager.rx;
    let sx = manager.sx.clone();
    let exit_message_id = manager.exit_message_id;

    tokio::task::spawn_blocking(move || {
        while let Ok(m) = sync_rx.recv() {
            if m.id() ==  exit_message_id {
                break;
            }
            if let Err(e) = sx.send(m.clone()) {
                log::error!("{:#?}", e);
            }
        }
    });

    let exit_message_id = manager.exit_message_id;

    while let Ok(msg) = rx.recv().await {
        if msg.id() == exit_message_id {
            if let Err(e) = sync_sx.send(msg.clone()) {
                log::error!("{:#?}", e);
            }
            let mut senders_inner = Vec::new();
            for (_, sender) in senders.0.read().await.iter() {
                senders_inner.push(sender.clone());
            }
            for s in senders_inner {
                let m = msg.clone();
                let _ = s.send(m);
            }
            break;
        }
        let s = match senders.0.read().await.get(&msg.id()) {
            Some(s) => s.clone(),
            None => {
                continue;
            }
        };

        if let Err(e) = s.send(msg) {
            log::error!("{:#?}", e);
        }
    }
}
