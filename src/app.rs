use crate::{
    agent::{Agent, Role},
    persistence::{self, Settings},
    wheel,
};
use eframe::egui::{self, Color32, RichText};
use rand::seq::SliceRandom;
use std::time::Instant;

pub struct ValorouletteApp {
    agents: Vec<Agent>,
    settings: Settings,
    rotation: f32,
    spinning: bool,
    spin_started: Option<Instant>,
    spin_duration: f32,
    spin_start_rotation: f32,
    spin_target_rotation: f32,
    selected_agent: Option<String>,
    custom_name: String,
    status: String,
}

impl ValorouletteApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let settings = persistence::load_settings();
        Self {
            agents: persistence::load_agents(),
            settings,
            rotation: 0.0,
            spinning: false,
            spin_started: None,
            spin_duration: 4.0,
            spin_start_rotation: 0.0,
            spin_target_rotation: 0.0,
            selected_agent: None,
            custom_name: String::new(),
            status: "Prêt à lancer la roulette".into(),
        }
    }

    fn available_agents(&self) -> Vec<String> {
        self.agents.iter()
            .filter(|a| !self.settings.excluded_agents.contains(&a.name))
            .filter(|a| {
                let role = a.role.unwrap_or(Role::Unknown).label().to_string();
                !self.settings.excluded_roles.contains(&role)
            })
            .filter(|a| {
                self.settings.selected_agents.is_empty()
                    || self.settings.selected_agents.contains(&a.name)
            })
            .map(|a| a.name.clone())
            .collect()
    }

    fn save(&self) {
        persistence::save_settings(&self.settings);
    }

    fn start_spin(&mut self) {
        let choices = self.available_agents();
        if choices.is_empty() {
            self.status = "Aucun agent disponible : vérifie les filtres.".into();
            return;
        }
        let chosen = if self.settings.secret_mode
            && !self.settings.secret_target.is_empty()
            && choices.contains(&self.settings.secret_target)
        {
            Some(self.settings.secret_target.clone())
        } else {
            choices.choose(&mut rand::rng()).cloned()
        };

        self.selected_agent = chosen;
        let target_index = self.selected_agent.as_ref()
            .and_then(|name| choices.iter().position(|candidate| candidate == name))
            .unwrap_or(0);
        let segment = std::f32::consts::TAU / choices.len() as f32;
        let desired_modulo = (-((target_index as f32) * segment) - segment * 0.5)
            .rem_euclid(std::f32::consts::TAU);
        let current_modulo = self.rotation.rem_euclid(std::f32::consts::TAU);
        let alignment = (desired_modulo - current_modulo).rem_euclid(std::f32::consts::TAU);

        self.spin_start_rotation = self.rotation;
        self.spin_target_rotation = self.rotation + 7.0 * std::f32::consts::TAU + alignment;
        self.spin_started = Some(Instant::now());
        self.spin_duration = 4.2;
        self.spinning = true;
        self.status = "La roulette tourne…".into();
    }

    fn update_animation(&mut self, ctx: &egui::Context) {
        if !self.spinning { return; }
        let elapsed = self.spin_started.map(|t| t.elapsed().as_secs_f32()).unwrap_or(0.0);
        let progress = (elapsed / self.spin_duration).clamp(0.0, 1.0);
        let eased = 1.0 - (1.0 - progress).powi(4);
        self.rotation = self.spin_start_rotation
            + (self.spin_target_rotation - self.spin_start_rotation) * eased;
        if progress >= 1.0 {
            self.spinning = false;
            self.status = self.selected_agent.as_ref()
                .map(|name| format!("Agent sélectionné : {name}"))
                .unwrap_or_else(|| "Aucun agent sélectionné".into());
        } else {
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        }
    }
}

impl eframe::App for ValorouletteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_animation(ctx);
        let available = self.available_agents();

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(RichText::new("VALOROULETTE").color(Color32::from_rgb(255, 70, 85)).strong());
                ui.label(RichText::new("ÉDITION RUST").small().color(Color32::GRAY));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{} agents disponibles", available.len()));
                });
            });
        });

        egui::SidePanel::left("settings").resizable(true).default_width(270.0).show(ctx, |ui| {
            ui.heading("Filtres");
            ui.checkbox(&mut self.settings.roles_visible, "Afficher les rôles");
            if self.settings.roles_visible {
                ui.separator();
                for role in Role::ALL {
                    let label = role.label().to_string();
                    let mut excluded = self.settings.excluded_roles.contains(&label);
                    if ui.checkbox(&mut excluded, label.clone()).changed() {
                        if excluded && !self.settings.excluded_roles.contains(&label) {
                            self.settings.excluded_roles.push(label.clone());
                        } else {
                            self.settings.excluded_roles.retain(|r| r != &label);
                        }
                        self.save();
                    }
                }
            }

            ui.separator();
            ui.heading("Agents exclus");
            egui::ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                for agent in &self.agents {
                    let mut excluded = self.settings.excluded_agents.contains(&agent.name);
                    if ui.checkbox(&mut excluded, &agent.name).changed() {
                        if excluded {
                            self.settings.excluded_agents.push(agent.name.clone());
                        } else {
                            self.settings.excluded_agents.retain(|n| n != &agent.name);
                        }
                        self.save();
                    }
                }
            });

            ui.separator();
            ui.heading("Agent personnalisé");
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.custom_name).hint_text("Nom de l'agent"));
                if ui.button("Ajouter").clicked() {
                    let name = self.custom_name.trim().to_string();
                    if !name.is_empty() && !self.agents.iter().any(|a| a.name.eq_ignore_ascii_case(&name)) {
                        self.agents.push(Agent {
                            name: name.clone(),
                            image_path: String::new(),
                            role: Some(Role::Unknown),
                            custom: true,
                        });
                        self.custom_name.clear();
                        self.status = format!("{name} ajouté (portrait à associer ultérieurement).");
                    }
                }
            });

            ui.separator();
            ui.checkbox(&mut self.settings.secret_mode, "Secret Mode");
            if self.settings.secret_mode {
                ui.label("Agent garanti");
                egui::ComboBox::from_id_salt("secret_target")
                    .selected_text(if self.settings.secret_target.is_empty() { "Choisir un agent" } else { &self.settings.secret_target })
                    .show_ui(ui, |ui| {
                        for name in &available {
                            ui.selectable_value(&mut self.settings.secret_target, name.clone(), name);
                        }
                    });
            }
            if ui.button("Enregistrer les paramètres").clicked() {
                self.save();
                self.status = "Paramètres enregistrés.".into();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);
                ui.label(RichText::new("QUI SERA TON AGENT ?").size(23.0).strong());
                ui.add_space(10.0);
                wheel::draw(ui, &available, self.rotation, 460.0);
                ui.add_space(14.0);

                if self.spinning {
                    ui.add_enabled(false, egui::Button::new("LA ROULETTE TOURNE…").min_size([220.0, 48.0].into()));
                } else if ui.add(egui::Button::new(RichText::new("LANCER LA ROULETTE").size(18.0).strong())
                    .fill(Color32::from_rgb(255, 70, 85))
                    .min_size([220.0, 48.0].into())).clicked() {
                    if self.settings.secret_mode && !self.settings.secret_target.is_empty() {
                        self.selected_agent = Some(self.settings.secret_target.clone());
                    }
                    self.start_spin();
                }

                ui.add_space(10.0);
                ui.label(RichText::new(&self.status).color(Color32::from_rgb(225, 225, 230)).size(16.0));
                if let Some(name) = &self.selected_agent {
                    ui.label(RichText::new(name).size(25.0).strong().color(Color32::from_rgb(255, 90, 105)));
                }
            });
        });

        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Valoroulette Rust — première base fonctionnelle").small().color(Color32::GRAY));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.small_button("Réinitialiser les exclusions").clicked() {
                        self.settings.excluded_agents.clear();
                        self.settings.excluded_roles.clear();
                        self.save();
                    }
                });
            });
        });

        if ctx.input(|i| i.viewport().close_requested()) {
            self.save();
        }
    }
}
