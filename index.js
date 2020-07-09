const game = import("./pkg/snake_game")

window.addEventListener("load", () => {
  game.then(code => {
    code.run();
  })
})

