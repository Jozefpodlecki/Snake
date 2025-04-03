
export interface LeaderboardEntry {
    name: string;
    score: number;
}

export const getLeaderboard = async (): Promise<LeaderboardEntry[]> => {
    let url;

    if(import.meta.env.DEV) {
        url = "https://localhost:5173/leaderboard.json"
    }
    else {
        url = "https://raw.githubusercontent.com/Jozefpodlecki/Snake/refs/heads/master/web/public/leaderboard.json";
    }

    const response = await fetch(url);
    const json = response.json();
    
    return json;
}

