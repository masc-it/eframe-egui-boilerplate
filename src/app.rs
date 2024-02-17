use async_process::Command;
use egui::Context;
use flume::{Receiver, Sender};
use std::sync::Arc;
use sync_cow::SyncCow;

use crate::{macros::{get_state, tokio_sleep, update_state, use_mut_state}, state::AppState};
pub struct TemplateApp {
    tx: Sender<(UpdateEvent, String)>,
    rx: Receiver<(UpdateEvent, String)>,
    state: Arc<SyncCow<AppState>>,
    ctx: Option<Context>,
}

#[derive(Debug)]
enum UpdateEvent {
    ShellOutput,
    StartStop,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let (tx, rx) = flume::unbounded();

        let app = Self {
            tx,
            rx,
            state: Arc::new(SyncCow::new(AppState::new())),
            ctx: None,
        };
        return app;
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app: TemplateApp = Default::default();
        app.ctx = Some(cc.egui_ctx.clone());
        app.update_state();

        app
    }

    fn update_state(&self) {
        let rx = self.rx.clone();
        let state = self.state.clone();

        let ctx = self.get_ctx();
        tokio::spawn(async move {
            let state = &*state;

            while let Ok(value) = rx.recv_async().await {
                match value {
                    (UpdateEvent::ShellOutput, v) => {
                        update_state!(state.output = v);
                    }
                    v => {
                        println!("{:?}", v);
                    }
                };
                ctx.request_repaint();
            }
        });
    }

    fn get_ctx(&self) -> egui::Context {
        return self.ctx.as_ref().unwrap().clone();
    }
    fn repeat_every_n_seconds(&self) {
        let tx2 = self.tx.clone();
        let state = Arc::clone(&self.state);

        tokio::spawn(async move {
            loop {
                let state = state.read();
                if !state.refresh {
                    println!("{}", state.refresh);
                    break;
                }

                println!("running cmd");
                let output = Command::new("./t.sh").output().await;

                if let Ok(r) = output {
                    let output_str = String::from_utf8_lossy(&r.stdout).to_string();
                    println!("{}", output_str);
                    tx2.send_async((UpdateEvent::ShellOutput, output_str))
                        .await
                        .expect("YO WHAAAAT");
                };

                tokio_sleep!(1000);
            }
            println!("REPEATING PROCESS END");
        });
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |_| {});
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (state, ro_state) = get_state!(self);

            ui.heading("APP");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.label(&ro_state.output);
            });

            use_mut_state!(ui.text_edit_multiline <= state.output);

            if ro_state.refresh == true {
                if ui.button("stop").clicked() {
                    update_state!(state.refresh = false);
                }
            } else {
                if ui.button("start").clicked() {
                    update_state!(state.refresh = true);
                    self.repeat_every_n_seconds();
                }
            }
        });
    }
}
