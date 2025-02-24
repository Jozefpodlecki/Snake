import Panel from "Panel";
import { useEffect, useState } from "react";
import init, { setup, stop, play, applyOptions } from "snake-game";
import Start from "Prompt";
import { GameOptions, GameState } from "types";

const defaultOptions: GameOptions = {
    id: "canvas",
    snakeColor: "#FFFFFF",
    difficulty: "hard",
    gridSize: 30,
    foodCount: 5,
    fps: 10,
    frameThresholdMs: 1000 / 10
};

function getOptions() {
    const json = localStorage.getItem("settings");
    const options = json ? JSON.parse(json) : defaultOptions;
    return options;
}

function saveOptionsToLocalStorage(options: GameOptions) {
    localStorage.setItem("settings", JSON.stringify(options));
}

function App() {
    const [options, setOptions] = useState(getOptions());
    const [state, setState] = useState<GameState>({ type: "loading" });

    useEffect(() => {
        onLoad();

        window.addEventListener("keydown", onKeyDown);

        return () => {
            stop();
        }

    }, []);

    function onKeyDown(event: KeyboardEvent) {
        
        if(event.code === "Space") {
            setState(state => {
                if(state.type === "playing") {
                    return state;
                }

                play(false);
                return {
                    ...state,
                    type: "playing",
                    score: 0
                }
            });
        }
    }

    async function onLoad() {
        await init();
        setup(options, onScore, onGameOver);
        play(true);
        setState(state => {
            return {
                ...state,
                type: "start-prompt"
            } as GameState
        });
    }

    function onGameOver() {
        setState(state => {
            return {
                ...state,
                type: "game-over"
            } as GameState
        });
    }

    function onScore() {
        setState(state => {
            if(state.type === "playing") {
                return {
                    ...state,
                    score: state.score + 1
                }
            }

            return state;
        });
    }

    function onToggle() {
        setState(state => {
            if(state.type === "settings") {
                play(false);
                return {
                    ...state,
                    type: "playing"
                }
            }

            stop();
            return {
                ...state,
                type: "settings"
            } as GameState
        });
    }

    function onOptionChange(options: GameOptions) {
        setOptions(options);
        saveOptionsToLocalStorage(options);
        applyOptions(options);
    }

    function renderWidgets(state: GameState) {
        switch(state.type) {
            case "start-prompt":
            case "game-over":
               return <Start state={state}/>
            case "playing":
            case "settings":
                return <>
                    <div className="absolute top-0 right-0 p-2">Score {state.score}</div>
                    <Panel
                        isOpen={state.type === "settings"}
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