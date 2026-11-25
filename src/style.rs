use eframe::egui::{self, Color32, RichText, Stroke};

// Global Color Palette
pub const BG_PANEL: Color32 = Color32::from_rgb(20, 24, 36);
pub const BG_CARD: Color32 = Color32::from_rgb(26, 31, 46);
pub const BG_CARD_SOFT: Color32 = Color32::from_rgb(22, 26, 38);
pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(42, 48, 64);
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(232, 236, 245);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(138, 144, 164);

// Accent Colors
pub const ACCENT_RED: Color32 = Color32::from_rgb(255, 70, 85);
pub const ACCENT_RED_HOVER: Color32 = Color32::from_rgb(255, 95, 108);
pub const ACCENT_CYAN: Color32 = Color32::from_rgb(0, 209, 255);
pub const ACCENT_CYAN_HOVER: Color32 = Color32::from_rgb(80, 225, 255);
pub const ACCENT_CYAN_TEXT: Color32 = Color32::from_rgb(10, 15, 25);
pub const ACCENT_VIOLET: Color32 = Color32::from_rgb(124, 92, 255);
pub const ACCENT_VIOLET_HOVER: Color32 = Color32::from_rgb(150, 122, 255);

// Action Colors
pub const ACCENT_GREEN: Color32 = Color32::from_rgb(0, 230, 118);
pub const ACCENT_GREEN_HOVER: Color32 = Color32::from_rgb(50, 245, 140);
pub const ACCENT_GREEN_TEXT: Color32 = Color32::from_rgb(10, 20, 15);

// Disabled State
pub const DISABLED_BG: Color32 = Color32::from_rgb(30, 35, 50);
pub const DISABLED_FG: Color32 = Color32::from_rgb(140, 150, 170);

/// Apply the global dark theme mimicking the original Electron app.
pub fn apply_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = BG_PANEL;
    visuals.window_fill = BG_CARD;
    visuals.extreme_bg_color = Color32::from_rgb(13, 15, 23);
    visuals.faint_bg_color = BG_CARD_SOFT;
    visuals.override_text_color = Some(TEXT_PRIMARY);
    
    visuals.widgets.noninteractive.bg_fill = BG_CARD;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, TEXT_PRIMARY);
    
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(35, 40, 56);
    visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(35, 40, 56);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, TEXT_PRIMARY);
    
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(50, 56, 76);
    visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(50, 56, 76);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
    
    visuals.widgets.active.bg_fill = ACCENT_RED;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
    
    visuals.selection.bg_fill = ACCENT_RED;
    visuals.hyperlink_color = ACCENT_RED;
    visuals.collapsing_header_frame = false;
    ctx.set_visuals(visuals);
}

/// Wrapper frame for sections to give them a card-like appearance.
pub fn section_card() -> egui::Frame {
    egui::Frame::default()
        .fill(BG_CARD_SOFT)
        .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
        .inner_margin(10.0)
        .corner_radius(egui::CornerRadius::same(10))
}

/// Custom styled action button with perfect text centering and custom colors.
pub fn action_button(
    ui: &mut egui::Ui,
    label: &str,
    fill: Color32,
    fill_hover: Color32,
    text_color: Color32,
    disabled_fill: Color32,
    disabled_text: Color32,
    enabled: bool,
) -> bool {
    let width = ui.available_width();
    let height = 52.0;
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click());
    
    let final_text_color = if enabled { text_color } else { disabled_text };
    
    let bg_color = if !enabled {
        disabled_fill
    } else if response.hovered() {
        fill_hover
    } else {
        fill
    };

    ui.painter().rect_filled(rect, egui::CornerRadius::same(12), bg_color);

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(18.0),
        final_text_color,
    );

    if enabled && response.hovered() {
        ui.painter().rect_stroke(
            rect,
            egui::CornerRadius::same(12),
            Stroke::new(2.0_f32, Color32::WHITE),
            egui::StrokeKind::Inside,
        );
    }

    enabled && response.clicked()
}

/// Secondary button to restore all excluded agents.
pub fn restore_all_button(ui: &mut egui::Ui, label: &str) -> bool {
    let width = ui.available_width();
    let height = 32.0;
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click());

    let bg_color = if response.hovered() {
        Color32::from_rgb(45, 52, 70)
    } else {
        Color32::TRANSPARENT
    };
    
    let stroke = if response.hovered() {
        Stroke::new(1.0_f32, Color32::WHITE)
    } else {
        Stroke::new(1.0_f32, BORDER_SUBTLE)
    };
    
    let text_color = if response.hovered() {
        Color32::WHITE
    } else {
        TEXT_PRIMARY
    };

    ui.painter().rect_filled(rect, egui::CornerRadius::same(8), bg_color);
    ui.painter().rect_stroke(
        rect,
        egui::CornerRadius::same(8),
        stroke,
        egui::StrokeKind::Inside,
    );
    
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(14.0),
        text_color,
    );

    response.clicked()
}

/// Primary "Add" button that stands out.
pub fn add_agent_button(ui: &mut egui::Ui, label: &str, enabled: bool) -> bool {
    let width = ui.available_width();
    let height = 40.0;
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click());
    
    let text_color = if enabled { ACCENT_GREEN_TEXT } else { DISABLED_FG };
    let bg_color = if !enabled {
        DISABLED_BG
    } else if response.hovered() {
        ACCENT_GREEN_HOVER
    } else {
        ACCENT_GREEN
    };

    ui.painter().rect_filled(rect, egui::CornerRadius::same(8), bg_color);
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(16.0),
        text_color,
    );

    enabled && response.clicked()
}

/// Destructive "Delete" button (red outline, red text).
pub fn delete_agent_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    let text = RichText::new(label).size(12.0).color(ACCENT_RED);
    
    let btn = egui::Button::new(text)
        .fill(Color32::TRANSPARENT)
        .stroke(Stroke::new(1.0_f32, ACCENT_RED))
        .corner_radius(egui::CornerRadius::same(6))
        .min_size(egui::vec2(70.0, 24.0));

    ui.scope(|ui| {
        ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
        ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, ACCENT_RED);
        ui.style_mut().visuals.widgets.hovered.bg_fill = ACCENT_RED;
        ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
        ui.style_mut().visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
        
        ui.add(btn)
    }).inner
}

/// Preview card showing what the new agent will look like.
pub fn agent_preview(
    ui: &mut egui::Ui,
    name: &str,
    role: &str,
    image: Option<&egui::TextureHandle>,
) {
    let frame = egui::Frame::default()
        .fill(BG_CARD)
        .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
        .inner_margin(10.0)
        .corner_radius(egui::CornerRadius::same(10));

    frame.show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.horizontal(|ui| {
            let avatar_size = 48.0;
            
            if let Some(tex) = image {
                let (rect, _) = ui.allocate_exact_size(egui::Vec2::splat(avatar_size), egui::Sense::hover());
                let radius = avatar_size / 2.0;
                let center = rect.center();
                
                let segments = 32usize;
                let mut mesh = egui::Mesh::with_texture(tex.id());
                let center_idx = mesh.vertices.len() as u32;
                mesh.vertices.push(egui::epaint::Vertex {
                    pos: center,
                    uv: egui::pos2(0.5, 0.5),
                    color: Color32::WHITE,
                });
                for i in 0..segments {
                    let angle = std::f32::consts::TAU * i as f32 / segments as f32;
                    let offset = egui::Vec2::angled(angle) * radius;
                    let uv = egui::pos2(0.5 + 0.5 * angle.cos(), 0.5 + 0.5 * angle.sin());
                    mesh.vertices.push(egui::epaint::Vertex {
                        pos: center + offset,
                        uv,
                        color: Color32::WHITE,
                    });
                }
                for i in 0..segments {
                    let next = (i + 1) % segments;
                    mesh.indices.push(center_idx);
                    mesh.indices.push(center_idx + 1 + i as u32);
                    mesh.indices.push(center_idx + 1 + next as u32);
                }
                ui.painter().add(egui::Shape::mesh(mesh));
            } else {
                let (rect, _) = ui.allocate_exact_size(egui::Vec2::splat(avatar_size), egui::Sense::hover());
                ui.painter().circle_filled(rect.center(), avatar_size / 2.0, BG_CARD_SOFT);
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "?",
                    egui::FontId::proportional(24.0),
                    TEXT_SECONDARY,
                );
            }
            
            ui.add_space(12.0);
            
            ui.vertical(|ui| {
                let display_name = if name.is_empty() { "..." } else { name };
                let name_color = if name.is_empty() { TEXT_SECONDARY } else { TEXT_PRIMARY };
                
                ui.label(RichText::new(display_name).size(16.0).strong().color(name_color));
                ui.label(RichText::new(role).size(12.0).color(TEXT_SECONDARY));
            });
        });
    });
}

/// Custom card for displaying excluded agents (clickable to restore).
pub fn excluded_card(
    ui: &mut egui::Ui,
    name: &str,
    portrait: Option<&egui::TextureHandle>,
) -> egui::Response {
    let frame = egui::Frame::default()
        .fill(BG_CARD)
        .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
        .inner_margin(8.0)
        .corner_radius(egui::CornerRadius::same(10));

    let inner = frame.show(ui, |ui| {
        ui.set_width(ui.available_width());
        
        ui.horizontal(|ui| {
            let avatar_size = 40.0;
            if let Some(tex) = portrait {
                ui.image((tex.id(), egui::Vec2::splat(avatar_size)));
            } else {
                ui.add_space(avatar_size);
            }
            ui.add_space(8.0);
            
            ui.add(
                egui::Label::new(RichText::new(name).size(15.0).strong().color(TEXT_PRIMARY))
                    .truncate()
            );
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(RichText::new("↺").size(20.0).color(ACCENT_RED));
            });
        });
    });

    let response = inner.response.interact(egui::Sense::click());

    if response.hovered() {
        ui.painter().rect_stroke(
            inner.response.rect,
            egui::CornerRadius::same(10),
            Stroke::new(2.0_f32, ACCENT_RED),
            egui::StrokeKind::Inside,
        );
    }

    response
}

/// Clean white button for the update banner (stands out on colored backgrounds).
pub fn banner_download_button(ui: &mut egui::Ui, label: &str, enabled: bool) -> bool {
    let width = ui.available_width();
    let height = 32.0;
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click());

    let text_color = if enabled { Color32::BLACK } else { Color32::GRAY };
    let bg_color = if !enabled {
        Color32::from_rgb(200, 200, 200)
    } else if response.hovered() {
        Color32::WHITE
    } else {
        Color32::from_rgb(240, 240, 240)
    };

    ui.painter().rect_filled(rect, egui::CornerRadius::same(6), bg_color);
    
    if enabled && response.hovered() {
        ui.painter().rect_stroke(
            rect,
            egui::CornerRadius::same(6),
            Stroke::new(1.0_f32, Color32::BLACK),
            egui::StrokeKind::Inside,
        );
    }

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(14.0),
        text_color,
    );

    enabled && response.clicked()
}