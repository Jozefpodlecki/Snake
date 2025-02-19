export interface GameOptions {
    id: string;
    difficulty: "easy" | "hard",
    gridSize: number;
    foodCount: number;
    fps: number;
    frameThresholdMs: number;
};