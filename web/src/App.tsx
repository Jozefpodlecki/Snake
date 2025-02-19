import Panel from "Panel";
import { useEffect, useState } from "react";
import init, { run, pause, play, apply_options } from "snake-game";
import { GameOptions } from "types";

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
    const [isInitialized, setInitialized] = useState(false);
    const [isOpen, setOpen] = useState(false);

    useEffect(() => {
        onLoad();

        return () => {
            stop();
        }
    }, []);

    useEffect(() => {

        if(!isInitialized) {
            return;
        }

        if (isOpen) {
            pause();
        }
        else {
            play();
        }

    }, [isInitialized, isOpen]);

    async function onLoad() {
        await init();
        setInitialized(true);

        (window as any).setScore = () => setScore(value => value + 1);

        run(options, onScore, onGameOver);
    }

    function onGameOver() {
        setScore(0);
        pause();
    }

    function onScore() {
        setScore(score => score + 1);
    }

    function onToggle() {
        setOpen(value => !value);
    }

    function onOptionChange(options: GameOptions) {
        setOptions(options);
        apply_options(options);
    }

    return <div className="app size-full">
        <canvas id="canvas"></canvas>
        <div className="absolute top-0 right-0 p-2">Score {score}</div>
        <Panel
            isOpen={isOpen}
            onToggle={onToggle}
            options={options}
            onOptionChange={onOptionChange}/>
    </div>
}

export default App;