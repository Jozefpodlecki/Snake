import { FC, useEffect, useState } from "react";
import { GameState } from "types";

interface LeaderboardEntry {
    name: string;
    score: number;
}

const LEADERBOARD_URL = "https://raw.githubusercontent.com/your-repo/leaderboard.json";

interface Props {
    state: GameState;
}

const Start: FC<Props> = ({state}) => {
    const [prompt, setPrompt] = useState("Press space to start");
    const [leaderboard, setLeaderboard] = useState<LeaderboardEntry[]>([
        { name: "Player1", score: 100 },
        { name: "Player2", score: 90 },
        { name: "Player3", score: 80 }
    ]);
    
    useEffect(() => {
        const updater = () => {
            setPrompt(prompt => prompt.includes("...") ? prompt.slice(0, -3) : prompt + ".");
        }

        const handle = setTimeout(updater, 1000);
        
        return () => {
            clearTimeout(handle);
        }

    }, []);

    useEffect(() => {
        fetch(LEADERBOARD_URL)
            .then((res) => res.json())
            .then((data) => setLeaderboard(data))
            .catch((err) => console.error("Failed to fetch leaderboard:", err));
    }, []);
     
    return <>
        <div className={`z-3 absolute bg-gray top-0 bg-[#000000AA] size-full flex justify-center items-center pointer-events-none`}>
            <div className="w-[600px]">
                {state == "game-over" ? <div className="mb-32">
                    <h1 className="text-center font-[sigmar] text-9xl text-[#CCCCCC]">Game Over!</h1>
                </div> : null}
                <div className="bg-opacity-10 p-4 rounded-2xl bg-opacity-20">
                    <h2 className="font-[sigmar] text-2xl text-white text-center mb-2">Leaderboard</h2>
                    <table className="w-full text-white text-lg border-collapse">
                        <thead>
                            <tr className="border-b border-white">
                                <th className="px-4 py-2 text-left">Rank</th>
                                <th className="px-4 py-2 text-left">Name</th>
                                <th className="px-4 py-2 text-left">Score</th>
                            </tr>
                        </thead>
                        <tbody>
                            {leaderboard.map((entry, index) => (
                                <tr key={index} className="border-b border-white">
                                    <td className="px-4 py-2">{index + 1}</td>
                                    <td className="px-4 py-2">{entry.name}</td>
                                    <td className="px-4 py-2">{entry.score}</td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>
                <div className="mt-16">
                    <h1 className="font-[sigmar] text-5xl text-[#CCCCCC] text-center">{prompt}</h1>
                </div>
            </div>
        </div>
    </>
}

export default Start;
