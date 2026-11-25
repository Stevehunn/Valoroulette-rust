use crate::{
    agent::{Agent, Role},
    i18n::Language,
    persistence::{self, Settings},
    style,
    wheel,
};
use eframe::egui::{self, Color32, RichText, TextEdit};
use rand::seq::IndexedRandom;
use std::collections::HashMap;
use std::time::Instant;

pub struct ValorouletteApp {
    agents: Vec<Agent>,
    portraits: HashMap<String, egui::TextureHandle>,
    settings: Settings,
    rotation: f32,
    spinning: bool,
    spin_started: Option<Instant>,
    spin_duration: f32,
    spin_start_rotation: f32,
    spin_target_rotation: f32,
    selected_agent: Option<String>,
    custom_name: String,
    custom_role_selection: Role,
    custom_role_text: String,
    custom_image_path: Option<String>,
    logo: Option<egui::TextureHandle>,
    preview_texture: Option<egui::TextureHandle>,
}

impl ValorouletteApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let settings = persistence::load_settings();
        let agents = persistence::load_agents();
        let mut portraits = HashMap::new();
        for agent in &agents {
            if !agent.image_path.is_empty() {
                if let Some(texture) = wheel::load_portrait(&cc.egui_ctx, &agent.image_path) {
                    portraits.insert(agent.name.clone(), texture);
                }
            }
        }
        let logo = wheel::load_portrait(&cc.egui_ctx, "assets/logo.png");
        style::apply_theme(&cc.egui_ctx);

        Self {
            agents,
            portraits,
            settings,
            rotation: 0.0,
            spinning: false,
            spin_started: None,
            spin_duration: 4.0,
            spin_start_rotation: 0.0,
            spin_target_rotation: 0.0,
            selected_agent: None,
            custom_name: String::new(),
            custom_role_selection: Role::Unknown,
            custom_role_text: String::new(),
            custom_image_path: None,
            logo,
            preview_texture: None,
        }
    }

    fn t(&self) -> &'static crate::i18n::Translations {
        self.settings.language.translations()
    }

    fn effective_role_key(&self, agent: &Agent) -> String {
        if !agent.custom_role.is_empty() {
            agent.custom_role.clone()
        } else {
            agent.role.unwrap_or(Role::Unknown).as_key().to_string()
        }
    }

    fn available_agents(&self) -> Vec<wheel::AgentPortrait> {
        self.agents
            .iter()
            .filter(|a| !self.settings.excluded_agents.contains(&a.name))
            .filter(|a| {
                let key = self.effective_role_key(a);
                !self.settings.excluded_roles.contains(&key)
            })
            .filter(|a| {
                self.settings.selected_agents.is_empty()
                    || self.settings.selected_agents.contains(&a.name)
            })
            .map(|a| wheel::AgentPortrait {
                name: a.name.clone(),
                image: self.portraits.get(&a.name).cloned(),
            })
            .collect()
    }

    fn all_role_keys(&self) -> Vec<String> {
        let mut roles: Vec<String> = Role::ALL.iter().map(|r| r.as_key().to_string()).collect();
        roles.push(Role::Unknown.as_key().to_string());
        let mut customs: Vec<String> = self
            .agents
            .iter()
            .filter(|a| !a.custom_role.is_empty())
            .map(|a| a.custom_role.clone())
            .collect();
        customs.sort();
        customs.dedup();
        customs.retain(|c| !roles.contains(c));
        roles.extend(customs);
        roles
    }

    fn persist(&self) {
        persistence::save_settings(&self.settings);
    }

    fn persist_agents(&self) {
        persistence::save_agents(&self.agents);
    }

    fn exclude_agent(&mut self, name: String) {
        if !self.settings.excluded_agents.contains(&name) {
            self.settings.excluded_agents.push(name.clone());
            self.persist();
        }
        if self.selected_agent.as_deref() == Some(name.as_str()) {
            self.selected_agent = None;
        }
    }

    fn restore_agent(&mut self, name: &str) {
        self.settings.excluded_agents.retain(|n| n != name);
        self.persist();
    }

    fn remove_agent(&mut self, name: &str) {
        self.agents.retain(|a| a.name != name);
        self.portraits.remove(name);
        self.settings.excluded_agents.retain(|n| n != name);
        if self.selected_agent.as_deref() == Some(name) {
            self.selected_agent = None;
        }
        self.persist_agents();
        self.persist();
    }

    fn add_agent(&mut self) {
        if self.custom_name.is_empty() {
            return;
        }
        let (role_opt, custom_role) = if !self.custom_role_text.is_empty() {
            (Some(Role::Unknown), self.custom_role_text.clone())
        } else {
            (Some(self.custom_role_selection), String::new())
        };
        let new_agent = Agent {
            name: self.custom_name.clone(),
            role: role_opt,
            custom: true,
            image_path: self.custom_image_path.clone().unwrap_or_default(),
            custom_role,
        };
        
        if let Some(texture) = self.preview_texture.clone() {
            self.portraits.insert(new_agent.name.clone(), texture);
        }
        
        self.agents.push(new_agent);
        self.custom_name.clear();
        self.custom_role_text.clear();
        self.custom_image_path = None;
        self.preview_texture = None;
        
        self.persist_agents();
        self.persist();
    }

    fn start_spin(&mut self, secret: bool) {
        let choices = self.available_agents();
        if choices.is_empty() {
            return;
        }

        let chosen = if secret
            && !self.settings.secret_target.is_empty()
            && choices.iter().any(|a| a.name == self.settings.secret_target)
        {
            Some(self.settings.secret_target.clone())
        } else {
            choices.choose(&mut rand::rng()).map(|a| a.name.clone())
        };

        self.selected_agent = chosen;
        let target_index = self
            .selected_agent
            .as_ref()
            .and_then(|name| choices.iter().position(|a| &a.name == name))
            .unwrap_or(0);

        let segment = std::f32::consts::TAU / choices.len() as f32;
        let target_angle = -std::f32::consts::FRAC_PI_2;
        let current_agent_angle = self.rotation + target_index as f32 * segment;
        let alignment = (target_angle - current_agent_angle).rem_euclid(std::f32::consts::TAU);

        self.spin_start_rotation = self.rotation;
        self.spin_target_rotation = self.rotation + 7.0 * std::f32::consts::TAU + alignment;
        self.spin_started = Some(Instant::now());
        self.spin_duration = 4.2;
        self.spinning = true;
    }

    fn update_animation(&mut self, ctx: &egui::Context) {
        if !self.spinning {
            return;
        }
        let elapsed = self
            .spin_started
            .map(|t| t.elapsed().as_secs_f32())
            .unwrap_or(0.0);
        let progress = (elapsed / self.spin_duration).clamp(0.0, 1.0);
        let eased = 1.0 - (1.0 - progress).powi(4);
        self.rotation = self.spin_start_rotation
            + (self.spin_target_rotation - self.spin_start_rotation) * eased;

        if progress >= 1.0 {
            self.spinning = false;
        } else {
            ctx.request_repaint();
        }
    }
}

impl eframe::App for ValorouletteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_animation(ctx);

        egui::SidePanel::left("controls")
            .resizable(true)
            .default_width(360.0)
            .width_range(300.0..=500.0)
            .show(ctx, |ui| {
                // Top Logo & Language Selector
                if let Some(logo) = &self.logo {
                    ui.add_space(12.0);
                    ui.vertical_centered(|ui| {
                        let w = ui.available_width() * 0.55;
                        ui.image((logo.id(), egui::vec2(w, w)));
                    });
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(6.0);
                }

                ui.horizontal(|ui| {
                    ui.label(self.t().language);
                    egui::ComboBox::from_id_salt("lang_pick")
                        .selected_text(self.settings.language.label())
                        .show_ui(ui, |ui| {
                            for lang in Language::ALL {
                                if ui.selectable_value(&mut self.settings.language, lang, lang.label()).changed() {
                                    self.persist();
                                }
                            }
                        });
                });
                ui.add_space(6.0);

                // Bottom Action Buttons (Anchored)
                egui::TopBottomPanel::bottom("side_buttons").show_inside(ui, |ui| {
                    ui.add_space(6.0);
                    // Removed explicit ui.separator() here to avoid double line
                    ui.add_space(8.0);

                    if style::restore_all_button(ui, self.t().restore_all) {
                        self.settings.excluded_agents.clear();
                        self.persist();
                    }

                    ui.add_space(8.0);

                    let exclude_enabled = !self.spinning && self.selected_agent.is_some();
                    if style::action_button(
                        ui,
                        self.t().exclude_btn,
                        style::ACCENT_CYAN,
                        style::ACCENT_CYAN_HOVER,
                        style::ACCENT_CYAN_TEXT,
                        style::DISABLED_BG,
                        style::DISABLED_FG,
                        exclude_enabled,
                    ) {
                        if let Some(name) = self.selected_agent.clone() {
                            self.exclude_agent(name);
                        }
                    }

                    ui.add_space(8.0);

                    let secret_modifier = ui.input(|i| {
                        i.modifiers.ctrl || i.modifiers.alt || i.modifiers.command
                    });
                    let (label, fill, hover, text) = if secret_modifier {
                        (
                            self.t().spin_secret_btn,
                            style::ACCENT_VIOLET,
                            style::ACCENT_VIOLET_HOVER,
                            Color32::WHITE,
                        )
                    } else {
                        (
                            self.t().spin_btn,
                            style::ACCENT_RED,
                            style::ACCENT_RED_HOVER,
                            Color32::WHITE,
                        )
                    };
                    if style::action_button(
                        ui,
                        label,
                        fill,
                        hover,
                        text,
                        style::DISABLED_BG,
                        style::DISABLED_FG,
                        true,
                    ) {
                        self.start_spin(secret_modifier);
                    }

                    ui.add_space(4.0);
                });

                // Scrollable Content
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .max_width(ui.available_width())
                    .show(ui, |ui| {
                        style::section_card().show(ui, |ui| {
                            egui::CollapsingHeader::new(
                                RichText::new(self.t().mode_secret).size(15.0).strong(),
                            )
                            .default_open(false)
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(self.t().secret_hint)
                                        .small()
                                        .color(style::TEXT_SECONDARY),
                                );
                                ui.add_space(4.0);
                                ui.horizontal(|ui| {
                                    ui.label(self.t().target);
                                    egui::ComboBox::from_id_salt("secret_target")
                                        .selected_text(if self.settings.secret_target.is_empty() {
                                            "—".to_string()
                                        } else {
                                            self.settings.secret_target.clone()
                                        })
                                        .show_ui(ui, |ui| {
                                            for agent in &self.agents {
                                                ui.selectable_value(
                                                    &mut self.settings.secret_target,
                                                    agent.name.clone(),
                                                    &agent.name,
                                                );
                                            }
                                        });
                                });
                            });
                        });
                        ui.add_space(6.0);

                        style::section_card().show(ui, |ui| {
                            egui::CollapsingHeader::new(
                                RichText::new(self.t().add_agent).size(15.0).strong(),
                            )
                            .default_open(false)
                            .show(ui, |ui| {
                                let hint_name = self.t().agent_name;
                                ui.add(
                                    TextEdit::singleline(&mut self.custom_name)
                                        .hint_text(hint_name),
                                );
                                ui.add_space(4.0);
                                ui.horizontal(|ui| {
                                    ui.label(self.t().role);
                                    egui::ComboBox::from_id_salt("role_pick")
                                        .selected_text(self.custom_role_selection.label(self.settings.language))
                                        .show_ui(ui, |ui| {
                                            for role in Role::ALL {
                                                ui.selectable_value(
                                                    &mut self.custom_role_selection,
                                                    role,
                                                    role.label(self.settings.language),
                                                );
                                            }
                                            ui.selectable_value(
                                                &mut self.custom_role_selection,
                                                Role::Unknown,
                                                Role::Unknown.label(self.settings.language),
                                            );
                                        });
                                });
                                ui.add_space(4.0);
                                let hint_role = self.t().custom_role;
                                ui.add(
                                    TextEdit::singleline(&mut self.custom_role_text)
                                        .hint_text(hint_role),
                                );
                                ui.add_space(4.0);
                                ui.horizontal(|ui| {
                                    if ui.button(self.t().choose_image).clicked() {
                                        if let Some(path) = rfd::FileDialog::new()
                                            .add_filter("Image PNG", &["png"])
                                            .pick_file()
                                        {
                                            self.custom_image_path = Some(path.to_string_lossy().to_string());
                                            self.preview_texture = wheel::load_portrait(ctx, &path.to_string_lossy());
                                        }
                                    }
                                    if let Some(filename) = &self.custom_image_path {
                                        ui.label(
                                            RichText::new(std::path::Path::new(filename).file_name().unwrap_or_default().to_string_lossy())
                                                .small()
                                                .color(style::TEXT_SECONDARY),
                                        );
                                    }
                                });
                                
                                ui.add_space(8.0);
                                
                                let display_role = if !self.custom_role_text.is_empty() {
                                    self.custom_role_text.clone()
                                } else {
                                    self.custom_role_selection.label(self.settings.language).to_string()
                                };
                                
                                style::agent_preview(
                                    ui, 
                                    &self.custom_name, 
                                    &display_role, 
                                    self.preview_texture.as_ref()
                                );

                                ui.add_space(8.0);
                                
                                if style::add_agent_button(ui, self.t().add_btn, !self.custom_name.is_empty()) {
                                    self.add_agent();
                                }
                            });
                        });
                        ui.add_space(6.0);

                        style::section_card().show(ui, |ui| {
                            egui::CollapsingHeader::new(
                                RichText::new(self.t().filters).size(15.0).strong(),
                            )
                            .default_open(false)
                            .show(ui, |ui| {
                                for role_key in self.all_role_keys() {
                                    let role_label = match role_key.as_str() {
                                        "duelist" => self.t().role_duelist,
                                        "initiator" => self.t().role_initiator,
                                        "controller" => self.t().role_controller,
                                        "sentinel" => self.t().role_sentinel,
                                        "unknown" => self.t().role_unknown,
                                        custom => custom,
                                    };
                                    let mut visible = !self.settings.excluded_roles.contains(&role_key);
                                    if ui.checkbox(&mut visible, role_label).changed() {
                                        if visible {
                                            self.settings.excluded_roles.retain(|r| r != &role_key);
                                        } else {
                                            self.settings.excluded_roles.push(role_key);
                                        }
                                        self.persist();
                                    }
                                }
                            });
                        });

                        ui.add_space(12.0);

                        // Excluded Agents List
                        ui.heading(RichText::new(self.t().excluded_agents).size(16.0).strong());
                        ui.add_space(6.0);
                        let excluded: Vec<Agent> = self
                            .agents
                            .iter()
                            .filter(|a| self.settings.excluded_agents.contains(&a.name))
                            .cloned()
                            .collect();

                        if excluded.is_empty() {
                            ui.label(
                                RichText::new(self.t().no_excluded_agent)
                                    .small()
                                    .color(style::TEXT_SECONDARY),
                            );
                        } else {
                            let mut to_restore: Option<String> = None;
                            for agent in &excluded {
                                let portrait = self.portraits.get(&agent.name);
                                if style::excluded_card(ui, &agent.name, portrait).clicked() {
                                    to_restore = Some(agent.name.clone());
                                }
                                ui.add_space(6.0);
                            }
                            if let Some(name) = to_restore {
                                self.restore_agent(&name);
                            }
                        }

                        // Custom Agents List
                        let custom_agents: Vec<Agent> = self
                            .agents
                            .iter()
                            .filter(|a| a.custom)
                            .cloned()
                            .collect();

                        if !custom_agents.is_empty() {
                            ui.add_space(12.0);
                            ui.heading(RichText::new(self.t().my_agents).size(16.0).strong());
                            ui.add_space(6.0);
                            let mut to_remove: Option<String> = None;
                            for agent in &custom_agents {
                                ui.horizontal(|ui| {
                                    if let Some(tex) = self.portraits.get(&agent.name) {
                                        ui.image((tex.id(), egui::Vec2::splat(28.0)));
                                    }
                                    let label = if agent.custom_role.is_empty() {
                                        agent.name.clone()
                                    } else {
                                        format!("{} ({})", agent.name, agent.custom_role)
                                    };
                                    
                                    ui.add(egui::Label::new(label).truncate());
                                    
                                    if style::delete_agent_button(ui, self.t().delete_btn).clicked() {
                                        to_remove = Some(agent.name.clone());
                                    }
                                });
                            }
                            if let Some(name) = to_remove {
                                self.remove_agent(&name);
                            }
                        }

                        ui.add_space(8.0);
                    });
            });

        // Central Wheel Panel
        let click_result = egui::CentralPanel::default()
            .show(ctx, |ui| {
                let available = self.available_agents();
                let diameter = ui.available_width().min(ui.available_height()) * 0.92;
                let winner_name = if self.spinning {
                    None
                } else {
                    self.selected_agent.as_deref()
                };
                let winner_image = winner_name.and_then(|n| self.portraits.get(n));
                wheel::draw(
                    ui,
                    &available,
                    self.rotation,
                    diameter,
                    winner_name,
                    winner_image,
                    self.t().no_agent_available,
                )
            })
            .inner;

        // Handle exclusion by clicking on the wheel
        if !self.spinning {
            if let Some(name) = click_result {
                self.exclude_agent(name);
            }
        }

        if ctx.input(|i| i.viewport().close_requested()) {
            self.persist();
        }
    }
}