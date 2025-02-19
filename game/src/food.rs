
pub struct Food {
    pub position: (i32, i32),
    cell_size: f32,
    spacing: f32,
    color: [f32; 4]
}

impl Food {
    pub fn new(color: [f32; 4], position: (i32, i32), cell_size: f32) -> Self {
        let spacing = 0.01;

        Food { 
            position,
            cell_size,
            spacing,
            color
        }
    }

    pub fn as_vertices(&self) -> Vec<f32> {
        let (x, y) = self.position;
        let x1 = x as f32 * self.cell_size - 1.0 + self.spacing;
        let y1 = y as f32 * self.cell_size - 1.0 + self.spacing;
        let x2 = x1 + self.cell_size - self.spacing;
        let y2 = y1 + self.cell_size - self.spacing;

        let vertices: [f32; 36] = [
            x1, y1, self.color[0], self.color[1], self.color[2], self.color[3],
            x2, y1, self.color[0], self.color[1], self.color[2], self.color[3],
            x1, y2, self.color[0], self.color[1], self.color[2], self.color[3],
            x1, y2, self.color[0], self.color[1], self.color[2], self.color[3],
            x2, y1, self.color[0], self.color[1], self.color[2], self.color[3],
            x2, y2, self.color[0], self.color[1], self.color[2], self.color[3],
        ];

        vertices.to_vec()
    }
}
