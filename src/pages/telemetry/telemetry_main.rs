use crate::components::button::button;
use crate::components::telemetry_not_found::telemetry_not_found;
use crate::frontend::frontend_main::{SettingsProvider, StateProvider};
use crate::telemetry::{self, Telemetry, TelemetryGraphValueType};
use eframe::egui::*;

use std::sync::Arc;
use std::time::Duration;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TelemetryPage {
    pub cur_layout_index: usize,
    pub layouts: Vec<LayoutInfo>,
    pub first_entry: bool,

    #[serde(skip)]
    pub(super) cur_driver: (String, i32),
    #[serde(skip)]
    pub(super) ref_lap_override: Option<telemetry::Lap>,
    #[serde(skip)]
    pub(super) in_layout_edit_mode: bool,
    #[serde(skip)]
    pub(super) edit_mode_context: EditModeContext,
    #[serde(skip)]
    pub(super) show_delete_layout_dialog: bool,
    #[serde(skip)]
    pub(super) show_add_limit_dialog: bool,
    #[serde(skip)]
    pub(super) save_as_dialog_info: SaveAsDialogInfo,
    #[serde(skip)]
    pub(super) settings_provider: Arc<SettingsProvider>,
    #[serde(skip)]
    pub(super) state_provider: Arc<StateProvider>,
    #[serde(skip)]
    pub(super) telemetry_provider: Arc<Option<Telemetry>>,
}

#[derive(Clone, Debug, Default)]
pub(super) struct EditModeContext {
    pub(super) layout: EditLayoutInfo,
    #[expect(unused)]
    pub(super) started_edtiting: bool,
}

#[derive(Clone, Debug, Default)]
pub(super) struct SaveAsDialogInfo {
    pub(super) name: String,
    pub(super) show: bool,
}

#[derive(Clone, Debug, Default)]
pub(super) struct EditLayoutInfo {
    pub(super) index: usize,
    pub(super) graphs: Vec<GraphInfo>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
#[serde(default)]
pub struct LayoutInfo {
    pub(super) name: String,
    pub(super) graphs: Vec<GraphInfo>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Clone, Debug, Default)]
pub(super) struct GraphInfo {
    pub(super) color: Color32,
    pub(super) show_ref: bool,
    pub(super) n_gridlines: i32,
    pub(super) size_percent: f32,
    pub(super) ref_val_type: TelemetryGraphValueType,
}
#[derive(Clone, Debug)]
pub(super) struct Lap<'a> {
    pub(super) values: &'a Vec<f32>,
    pub(super) distances: &'a Vec<f32>,
}

#[derive(Clone, Debug)]
pub(super) struct DynGraphData<'a> {
    pub(super) cur_lap: Lap<'a>,
    pub(super) ref_lap: Lap<'a>,
}

impl TelemetryPage {
    pub fn new(
        settings_provider: Arc<SettingsProvider>,
        state_provider: Arc<StateProvider>,
        telemetry_provider: Arc<Option<Telemetry>>,
    ) -> Self {
        Self {
            first_entry: true,
            settings_provider,
            state_provider,
            telemetry_provider,
            ref_lap_override: None,
            cur_driver: ("".to_owned(), 0),
            in_layout_edit_mode: false,
            show_delete_layout_dialog: false,
            show_add_limit_dialog: false,
            cur_layout_index: 0,
            save_as_dialog_info: SaveAsDialogInfo {
                name: "".to_owned(),
                show: false,
            },
            edit_mode_context: EditModeContext {
                layout: EditLayoutInfo {
                    index: 0,
                    graphs: vec![],
                },
                started_edtiting: false,
            },
            layouts: vec![LayoutInfo {
                name: "Main".to_owned(),
                graphs: vec![GraphInfo {
                    show_ref: false,
                    ref_val_type: TelemetryGraphValueType::Rpm,
                    color: Color32::WHITE,
                    n_gridlines: 3,
                    size_percent: 1.0,
                }],
            }],
        }
    }
}

impl TelemetryPage {
    pub fn draw_telemetry_page(&mut self, ui: &mut Ui) {
        ui.request_repaint_after(Duration::from_millis(16));

        if self.first_entry && !*self.state_provider.global_first_launch.read().unwrap() {
            self.draw_first_entry_dialog(ui);
        }

        if self.telemetry_provider.is_none() {
            if !self.first_entry {
                telemetry_not_found(ui);
            }
            return;
        }

        if !self.in_layout_edit_mode {
            self.draw_normal_mode(ui);
        } else {
            self.draw_edit_mode(ui);
        }
    }

    fn draw_first_entry_dialog(&mut self, ui: &mut Ui) {
        Window::new("Hello")
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ui.ctx(), |ui| {
                ui.set_min_size(vec2(500.0, 0.0));
                ui.vertical(|ui| {
                    ui.label(RichText::new("Welcome to the telemetry page, where you can view live telemetry data as you race. The dropdown on the top bar lets you select any car currently in the lobby, so you can follow your own data or keep an eye on another driver. Next to it, the layout dropdown lets you switch between layouts - two default layouts are available to start with.").size(18.0));
                    ui.label(RichText::new("If you want to customize what you see, click the pen icon in the top-right corner to enter edit mode. Once inside, the top bar shows you which layout you're currently editing, along with controls to save your changes, discard them, save the layout as a new one, or delete it entirely. You can also add new graphs using the Add Graph button on the top bar. Any graph on the page can be resized vertically, and its individual properties can be adjusted directly through the fields shown on the graph itself.").size(18.0));
                    if button(
                        ui,
                        vec2(64.0, 32.0),
                        CornerRadius::same(8),
                        Color32::from_white_alpha(25),
                        "Close",
                        FontId::new(16.0, FontFamily::Proportional),
                        Color32::WHITE,
                    )
                    .clicked()
                    {
                        self.first_entry = false;
                    }
                });
            });
    }
}
