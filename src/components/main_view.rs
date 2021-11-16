use crate::gui_app::{Filter, GuiAppState};
use egui::CtxRef;

pub fn main_view(app_state: &mut GuiAppState, ctx: &CtxRef, socket_address: &str) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading(format!("Listening on: {}", socket_address));

            egui::ScrollArea::vertical()
                .max_width(ui.available_width())
                .max_height(ui.available_height() - app_state.preview_height)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    egui::Grid::new("received_grid")
                        .striped(true)
                        .spacing((0.0, 10.0))
                        .min_col_width(ui.available_width() / 6.0)
                        .max_col_width(ui.available_width() / 6.0)
                        .show(ui, |ui| {
                            ui.heading("");
                            ui.heading("Message");
                            ui.heading("Host");
                            ui.heading("File name");
                            ui.heading("Line number");
                            ui.heading("Date & time");
                            ui.end_row();

                            let received_vec = app_state.received.read().unwrap();
                            let mut received_vec: Vec<_> = received_vec.iter().collect();

                            received_vec.sort_by(|(_, a_time), (_, b_time)| {
                                if app_state.is_newest_first {
                                    b_time.partial_cmp(a_time).unwrap()
                                } else {
                                    a_time.partial_cmp(b_time).unwrap()
                                }
                            });

                            for received in
                                received_vec.iter().filter(|(log, time)| match app_state
                                    .filter_by
                                {
                                    Filter::Message =>
                                        if app_state.is_case_sensitive {
                                            log.message.contains(&app_state.search_filter)
                                        } else {
                                            log.message.to_lowercase().contains(
                                                &app_state.search_filter.to_lowercase(),
                                            )
                                        },
                                    Filter::Time => time
                                        .format("%F %X")
                                        .to_string()
                                        .contains(&app_state.search_filter),
                                    Filter::FileName =>
                                        if app_state.is_case_sensitive {
                                            log.file_name
                                                .contains(&app_state.search_filter)
                                        } else {
                                            log.message.to_lowercase().contains(
                                                &app_state.search_filter.to_lowercase(),
                                            )
                                        },
                                    Filter::Address => false,
                                    Filter::LineNumber => {
                                        let number = app_state
                                            .search_filter
                                            .parse::<u32>()
                                            .unwrap_or(0);

                                        if number == 0 {
                                            return true;
                                        }

                                        log.line_number == number
                                    },
                                })
                            {
                                ui.horizontal(|ui| {
                                    if let Some(clicked_item) = &app_state.clicked_item {
                                        let _checked =
                                            ui.radio(*received == clicked_item, "");
                                    } else {
                                        let _checked = ui.radio(false, "");
                                    }

                                    // u1f50e = 🔎
                                    if ui.button("Examine \u{1f50e}").clicked() {
                                        app_state.clicked_item =
                                            Some((*received).clone());
                                    };
                                });

                                ui.label(&received.0.message.replace("\"", ""));
                                ui.label(&received.0.address);
                                ui.label(&received.0.file_name);
                                ui.label(&received.0.line_number);
                                ui.label(&received.1.format("%F %X"));
                                ui.end_row();
                            }
                        });
                });
        });
    });
}
