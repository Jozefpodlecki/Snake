pub struct Obstacle {
    pub position: (i32, i32),
    cell_size: f32,
    spacing: f32,
    color: [f32; 4],
}

impl Obstacle {
    pub fn new(color: [f32; 4], position: (i32, i32), cell_size: f32) -> Self {
        let spacing = 0.01;

        Obstacle {
            position,
            cell_size,
            spacing,
            color,
        }
    }

    pub fn as_vertices(&self) -> Vec<f32> {
        let (x, y) = self.position;
        let size = 2; // 2x2 block

        let mut all_vertices = Vec::new();

        for i in 0..size {
            for j in 0..size {
                let x1 = (x + i) as f32 * self.cell_size - 1.0 + self.spacing;
                let y1 = (y + j) as f32 * self.cell_size - 1.0 + self.spacing;
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

                all_vertices.extend_from_slice(&vertices);
            }
        }

        all_vertices
    }

    pub fn occupies(&self, position: (i32, i32)) -> bool {
        let (x, y) = self.position;
        position.0 >= x && position.0 < x + 2 && position.1 >= y && position.1 < y + 2
    }
}
