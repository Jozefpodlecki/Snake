import Panel from "Panel";
import { useEffect, useState } from "react";
import init, { setup, stop, play, applyOptions } from "snake-game";
import Start from "Start";
import { GameOptions, GameState } from "types";

const defaultOptions: GameOptions = {
    id: "canvas",
    difficulty: "easy",
    gridSize: 30,
    foodCount: 5,
    fps: 10,
    frameThresholdMs: 1000 / 10
};

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
            case "start-prompt":
                play(true);
                break;
            case "playing":
                play(false);
                break;
            case "settings":
                stop();
                break;
        }

    }, [state]);

    function onKeyDown(event: KeyboardEvent) {
        
        if(event.code === "Space") {
            setState(value => {
                return value === "playing" ? value : "playing"
            });
        }
    }

    async function onLoad() {
        await init();
        setup(options, onScore, onGameOver);
        setState("start-prompt");
    }

    function onGameOver() {
        setState("game-over");
        setScore(0);
        // stop();
    }

    function onScore() {
        setScore(score => score + 1);
    }

    function onToggle() {
        setState(state => state === "settings" ? "playing" : "settings");
    }

    function onOptionChange(options: GameOptions) {
        setOptions(options);
        applyOptions(options);
    }

    function renderWidgets(state: GameState) {
        switch(state) {
            case "start-prompt":
            case "game-over":
               return <Start state={state}/>
            case "playing":
            case "settings":
                return <>
                    <div className="absolute top-0 right-0 p-2">Score {score}</div>
                    <Panel
                        isOpen={state === "settings"}
                        onToggle={onToggle}
                        options={options}
                        onOptionChange={onOptionChange}/>
                </>
        }
    }

    return <div className="app size-full">
        <canvas id="canvas"></canvas>
        {renderWidgets(state)}
    </div>
}

export default App;