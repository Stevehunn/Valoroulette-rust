use eframe::egui::{self, Align2, Color32, FontId, Pos2, Shape, Stroke, Vec2};
use std::f32::consts::TAU;

const COLORS: [Color32; 8] = [
    Color32::from_rgb(255, 70, 85),
    Color32::from_rgb(32, 42, 58),
    Color32::from_rgb(190, 48, 64),
    Color32::from_rgb(48, 61, 82),
    Color32::from_rgb(125, 35, 50),
    Color32::from_rgb(63, 75, 96),
    Color32::from_rgb(160, 48, 60),
    Color32::from_rgb(40, 51, 69),
];

pub fn draw(ui: &mut egui::Ui, agents: &[String], rotation: f32, diameter: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(diameter), egui::Sense::hover());
    let painter = ui.painter_at(rect);
    let center = rect.center();
    let radius = diameter * 0.46;

    painter.circle_filled(center, radius + 5.0, Color32::from_rgb(12, 16, 24));
    if agents.is_empty() {
        painter.circle_filled(center, radius, Color32::from_rgb(35, 42, 55));
        painter.text(center, Align2::CENTER_CENTER, "Aucun agent", FontId::proportional(20.0), Color32::WHITE);
        return;
    }

    let step = TAU / agents.len() as f32;
    for (index, name) in agents.iter().enumerate() {
        let start = rotation + index as f32 * step - std::f32::consts::FRAC_PI_2;
        let end = start + step;
        let mut points = vec![center];
        let segments = ((step * 24.0).ceil() as usize).max(3);
        for n in 0..=segments {
            let angle = start + (end - start) * n as f32 / segments as f32;
            points.push(center + Vec2::angled(angle) * radius);
        }
        painter.add(Shape::convex_polygon(
            points,
            COLORS[index % COLORS.len()],
            Stroke::new(1.5, Color32::from_rgb(15, 19, 27)),
        ));

        let middle = start + step * 0.5;
        let label_pos = center + Vec2::angled(middle) * radius * 0.66;
        painter.text(
            label_pos,
            Align2::CENTER_CENTER,
            name,
            FontId::proportional(if agents.len() > 18 { 9.0 } else { 12.0 }),
            Color32::WHITE,
        );
    }

    painter.circle_filled(center, radius * 0.13, Color32::from_rgb(12, 16, 24));
    painter.circle_stroke(center, radius, Stroke::new(3.0, Color32::from_rgb(210, 215, 225)));

    // Pointer fixed at the top of the wheel.
    let tip = Pos2::new(center.x, center.y - radius - 1.0);
    let pointer = vec![
        tip,
        Pos2::new(tip.x - 13.0, tip.y - 25.0),
        Pos2::new(tip.x + 13.0, tip.y - 25.0),
    ];
    painter.add(Shape::convex_polygon(pointer, Color32::from_rgb(255, 70, 85), Stroke::new(1.0, Color32::WHITE)));
}
