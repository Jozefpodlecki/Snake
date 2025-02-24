import { render, screen } from '@testing-library/react'
import Panel from './Panel'

describe('Panel', () => {
	it('renders the Panel component', () => {

        const options = {
            id: "canvas",
            snakeColor: "#FFFFFF",
            difficulty: "hard",
            gridSize: 30,
            foodCount: 5,
            fps: 10,
            frameThresholdMs: 1000 / 10
        } as const;

        render(<Panel
            isOpen={true}
            onToggle={() => {}}
            options={options}
            onOptionChange={() => {}} />)

        screen.debug();
    })
})