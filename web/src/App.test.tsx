import { render, screen } from '@testing-library/react'
import App from './App'

vi.mock("snake-game", () => ({
	default: vi.fn().mockResolvedValue(undefined),
	setup: vi.fn(),
	stop: vi.fn(),
	play: vi.fn(),
	applyOptions: vi.fn(),
}));
  
describe('App', () => {
	it('renders the App component', () => {
	render(<App />)

	screen.debug();
	})
})