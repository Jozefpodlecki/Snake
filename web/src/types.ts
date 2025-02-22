export interface GameOptions {
    id: string;
    snakeColor: string;
    difficulty: "easy" | "hard",
    gridSize: number;
    foodCount: number;
    fps: number;
    frameThresholdMs: number;
};

export type GameState = {
    type: "loading";
} | {
    type: "start-prompt";
} | {
    type: "playing" | "settings" | "game-over";
    score: number;
}