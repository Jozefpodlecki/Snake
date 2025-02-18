import { useEffect, useState } from "react";
import init, { run, stop } from "snake-game";

function App() {
    const [score, setScore] = useState(0);

    useEffect(() => {
        onLoad();

        return () => {
            stop();
        }
    }, []);

    async function onLoad() {
        await init();

        const options = {
            id: "canvas",
            frame_threshold_ms: 122,
            onGameOver
        }

        run(options, onScore, onGameOver);
    }

    function onGameOver() {
        setScore(0);
    }

    function onScore() {
        setScore(score => score + 1);
    }

    return <div className="app size-full">
        <canvas id="canvas"></canvas>
        <div className="absolute top-0 right-0 p-2">Score {score}</div>
    </div>
}

export default App;