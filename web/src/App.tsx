import Panel from "Panel";
import { useEffect, useState } from "react";
import init, { run, pause, play, restart, apply_options } from "snake-game";
import Start from "Start";
import { GameOptions } from "types";

const defaultOptions: GameOptions = {
    id: "canvas",
    difficulty: "easy",
    gridSize: 30,
    foodCount: 5,
    fps: 10,
    frameThresholdMs: 1000 / 10
};

type GameState = "loading" | "start-prompt" | "playing" | "settings" | "game-over";

function App() {
    const [score, setScore] = useState(0);
    const [options, setOptions] = useState(defaultOptions);
    const [state, setState] = useState<GameState>("loading");

    useEffect(() => {
        onLoad();

        window.addEventListener("keydown", onKeyDown);

        return () => {
            stop();
        }

    }, []);

    useEffect(() => {

        switch(state) {
            case "playing":
                play();
                break;
            case "settings":
                pause();
                break;
        }

    }, [state]);

    function onKeyDown(event: KeyboardEvent) {
        
        if(event.code === "Space") {
            setState("playing");
            restart();
        }
    }

    async function onLoad() {
        await init();
        run(options, onScore, onGameOver);
        setState("start-prompt");
    }

    function onGameOver() {
        setScore(0);
        pause();
    }

    function onScore() {
        setScore(score => score + 1);
    }

    function onToggle() {
        setState(state => state === "settings" ? "playing" : "settings");
    }

    function onOptionChange(options: GameOptions) {
        setOptions(options);
        apply_options(options);
    }

    return <div className="app size-full">
        <canvas id="canvas"></canvas>
        {state === "start-prompt" ? null : <div className="absolute top-0 right-0 p-2">Score {score}</div>}
        {state === "start-prompt" ? <Start/> : null}
        <Panel
            isOpen={state === "settings"}
            onToggle={onToggle}
            options={options}
            onOptionChange={onOptionChange}/>
    </div>
}

export default App;