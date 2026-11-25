use crate::style;
use eframe::egui::{self, Align2, Color32, FontId, Pos2, Sense, Shape, Stroke, Ui, Vec2};
use std::f32::consts::TAU;
use std::path::Path;

const WHEEL_BG: Color32 = Color32::from_rgb(22, 27, 42);
const WHEEL_RING: Color32 = Color32::from_rgb(38, 44, 62);
const PORTRAIT_BG: Color32 = Color32::from_rgb(42, 48, 66);

pub struct AgentPortrait {
    pub name: String,
    pub image: Option<egui::TextureHandle>,
}

pub fn draw(
    ui: &mut Ui,
    agents: &[AgentPortrait],
    rotation: f32,
    diameter: f32,
    winner_name: Option<&str>,
    winner_image: Option<&egui::TextureHandle>,
    no_agent_text: &str,
) -> Option<String> {
    let size = Vec2::splat(diameter);
    let (response, painter) = ui.allocate_painter(size, Sense::click());
    let center = response.rect.center();
    let radius = diameter * 0.42;
    let portrait_radius = radius * 0.82;

    painter.circle_filled(
        center + Vec2::new(0.0, 10.0),
        radius + 22.0,
        Color32::from_rgba_unmultiplied(0, 0, 0, 90),
    );
    painter.circle_filled(
        center + Vec2::new(0.0, 6.0),
        radius + 20.0,
        Color32::from_rgba_unmultiplied(0, 0, 0, 60),
    );

    painter.circle_filled(center, radius + 18.0, WHEEL_BG);
    painter.circle_stroke(center, radius + 18.0, Stroke::new(1.0_f32, WHEEL_RING));
    painter.circle_stroke(center, radius + 14.0, Stroke::new(3.0_f32, style::ACCENT_RED));

    if agents.is_empty() {
        painter.text(
            center,
            Align2::CENTER_CENTER,
            no_agent_text,
            FontId::proportional(18.0),
            style::TEXT_SECONDARY,
        );
        return None;
    }

    let angle_step = TAU / agents.len() as f32;
    let portrait_size = (diameter * 0.115).max(32.0);
    let portrait_circle_radius = portrait_size * 0.55;
    let hover_pos = response.hover_pos();
    let click_pos = response.interact_pointer_pos();

    let mut hovered_index: Option<usize> = None;
    let mut best_dist = f32::MAX;
    if let Some(hover) = hover_pos {
        for (i, _) in agents.iter().enumerate() {
            let angle = rotation + i as f32 * angle_step;
            let pos = center + Vec2::angled(angle) * portrait_radius;
            let dist = pos.distance(hover);
            if dist <= portrait_circle_radius && dist < best_dist {
                best_dist = dist;
                hovered_index = Some(i);
            }
        }
    }

    for (i, agent) in agents.iter().enumerate() {
        let angle = rotation + i as f32 * angle_step;
        let pos = center + Vec2::angled(angle) * portrait_radius;
        let is_hovered = hovered_index == Some(i);

        if is_hovered {
            painter.circle_filled(
                pos,
                portrait_circle_radius + 4.0,
                Color32::from_rgba_unmultiplied(255, 70, 85, 60),
            );
        }
        painter.circle_filled(pos, portrait_circle_radius, PORTRAIT_BG);

        if let Some(texture) = &agent.image {
            draw_circular_image(&painter, texture, pos, portrait_circle_radius);
        } else {
            let initial = agent.name.chars().next().unwrap_or('?').to_string();
            painter.text(
                pos,
                Align2::CENTER_CENTER,
                initial,
                FontId::proportional(portrait_size * 0.5),
                style::TEXT_PRIMARY,
            );
        }

        let (ring_color, ring_width) = if is_hovered {
            (Color32::WHITE, 3.0_f32)
        } else {
            (style::ACCENT_RED, 2.0_f32)
        };
        painter.circle_stroke(pos, portrait_circle_radius, Stroke::new(ring_width, ring_color));
    }

    if let Some(name) = winner_name {
        let center_radius = radius * 0.38;
        painter.circle_filled(
            center,
            center_radius * 1.25,
            Color32::from_rgba_unmultiplied(255, 70, 85, 40),
        );
        painter.circle_filled(center, center_radius * 1.1, WHEEL_BG);
        painter.circle_stroke(center, center_radius * 1.1, Stroke::new(3.0_f32, style::ACCENT_RED));
        painter.circle_filled(center, center_radius, PORTRAIT_BG);

        if let Some(texture) = winner_image {
            draw_circular_image(&painter, texture, center, center_radius);
        } else {
            let initial = name.chars().next().unwrap_or('?').to_string();
            painter.text(
                center,
                Align2::CENTER_CENTER,
                initial,
                FontId::proportional(center_radius * 0.9),
                Color32::WHITE,
            );
        }

        let label_pos = center + Vec2::new(0.0, center_radius * 1.1 + 20.0);
        painter.text(
            label_pos,
            Align2::CENTER_CENTER,
            name,
            FontId::proportional(18.0),
            style::ACCENT_RED,
        );
    }

    let tip = Pos2::new(center.x, center.y - radius - 18.0);
    let pointer = vec![
        tip,
        Pos2::new(tip.x - 14.0, tip.y - 28.0),
        Pos2::new(tip.x + 14.0, tip.y - 28.0),
    ];
    painter.add(Shape::convex_polygon(
        pointer,
        style::ACCENT_RED,
        Stroke::new(1.0_f32, Color32::WHITE),
    ));

    if response.clicked() {
        if let Some(click) = click_pos {
            let mut best: Option<(usize, f32)> = None;
            for (i, _) in agents.iter().enumerate() {
                let angle = rotation + i as f32 * angle_step;
                let pos = center + Vec2::angled(angle) * portrait_radius;
                let dist = pos.distance(click);
                if dist <= portrait_circle_radius {
                    if best.map_or(true, |(_, d)| dist < d) {
                        best = Some((i, dist));
                    }
                }
            }
            if let Some((i, _)) = best {
                return Some(agents[i].name.clone());
            }
        }
    }

    None
}

fn draw_circular_image(
    painter: &egui::Painter,
    texture: &egui::TextureHandle,
    center: Pos2,
    radius: f32,
) {
    let segments = 48usize;
    let mut mesh = egui::Mesh::with_texture(texture.id());

    let center_idx = mesh.vertices.len() as u32;
    mesh.vertices.push(egui::epaint::Vertex {
        pos: center,
        uv: egui::pos2(0.5, 0.5),
        color: Color32::WHITE,
    });

    for i in 0..segments {
        let angle = TAU * i as f32 / segments as f32;
        let offset = Vec2::angled(angle) * radius;
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

    painter.add(Shape::mesh(mesh));
}

pub fn load_portrait(ctx: &egui::Context, path: &str) -> Option<egui::TextureHandle> {
    if !Path::new(path).exists() {
        return None;
    }
    let image = image::open(path).ok()?.to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
    Some(ctx.load_texture(path, color_image, egui::TextureOptions::LINEAR))
}