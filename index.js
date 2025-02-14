const onLoad = async () => {
	const game = await import("snake-game");

	game.run()
}

window.addEventListener("load", onLoad);