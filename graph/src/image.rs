// use eframe::epaint;

#[derive(Debug, Clone, Copy)]
pub struct Image {
    pub pos: egui::Pos2,
    size: egui::Vec2,
    texture_id: egui::TextureId,
}

impl Image {
    pub fn new(pos: egui::Pos2, size: egui::Vec2, texture_id: egui::TextureId) -> Self {
        Self {
            pos,
            size,
            texture_id,
        }
    }

    pub fn contains(&self, point: egui::Pos2) -> bool {
        self.rect().contains(point)
    }

    pub fn follow_mouse(&mut self, event: &egui::Event) {
        if let egui::Event::PointerMoved(pos) = event {
            self.pos = *pos;
        }
    }

    // pub fn rotate(&mut self, angle: f32){
    //    self.pos = self.pos.tint(egui::Pos2::ZERO, angle);
    // }

    pub fn rect(&self) -> egui::Rect {
        egui::Rect::from_min_size(self.pos - self.size / 2., self.size)
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        ui.put(self.rect(), egui::Image::new(self.texture_id, self.size));
    }
}
