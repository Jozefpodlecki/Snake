import { render, screen } from '@testing-library/react'
import Prompt from './Prompt'

vi.mock("api", () => ({
    getLeaderboard: vi.fn().mockResolvedValue([
        { name: "Peter", score: 100 },
        { name: "Steve", score: 80 },
    ]),
}));

describe('Prompt', () => {
	it('renders the Prompt component', () => {

        const state = {
            type: "game-over",
            score: 1,
        } as const;

        render(<Prompt state={state}/>)

        screen.debug();
    })
})