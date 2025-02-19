# 🐍 WebAssembly Snake Game

This is a **Snake game** built using **Rust** and **WebAssembly (WASM)**, with a **React frontend**. The game runs efficiently in the browser using **WebGL2** for rendering.

## 🚀 Features

- **WebAssembly-Powered**: The game logic is written in Rust and compiled to WebAssembly for high performance.
- **React Integration**: A React frontend handles game settings and user interactions.
- **WebGL2 Rendering**: Uses WebGL2 for smooth and efficient graphics.

---

## 🛠️ Technology Stack

- **Rust** → Game logic and WebAssembly bindings
- **WebAssembly (WASM)** → Runs the Rust code in the browser
- **React (TypeScript)** → Frontend UI
- **WebGL2** → Graphics rendering
- **JS/WASM Interop** → `wasm_bindgen` for Rust-JS communication

---

## 📜 How It Works

### **Game Loop**
1. The game initializes a WebGL2 rendering context.
2. The snake moves automatically, and the player controls its direction using the keyboard.
3. When the snake eats food, it grows in size.
4. If the snake collides with itself, the game resets.
5. The game runs inside a **requestAnimationFrame** loop for smooth performance.

---

## 🕹️ How to Play

1. Open the game in a browser.
2. Use **arrow keys** to move the snake.
3. Eat food to grow longer.
4. Avoid hitting the walls or yourself.
5. **Pause the game by opening the settings panel** (top-left **three-dot menu**).

---

## 📦 Installation & Setup

1. **Clone the repository**:

```sh
git clone https://github.com/your-repo/snake-game.git
cd snake-game
```

## 🛣️ Roadmap

### Planned Features
- 🎨 **Change Snake Color** – Allow users to customize the snake's color in the settings panel.


## 📜 Credits

- [rustwasm.github.io](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html)
- [developer.mozilla.org = Rust_to_Wasm](https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm)
- [rollup-starter-app](https://github.com/rollup/rollup-starter-app/blob/master/public/index.html)
- [wasm-bindgen - request animation frame](https://github.com/rustwasm/wasm-bindgen/issues/976)
- [so -how-to-convert-closure-to-js-sysfunction](https://stackoverflow.com/questions/60054963/how-to-convert-closure-to-js-sysfunction)
- [reactjs-vite-tailwindcss-boilerplate](https://github.com/joaopaulomoraes/reactjs-vite-tailwindcss-boilerplate/blob/main/index.html)
- [simpleicons](https://simpleicons.org/)
- [wasm-bindgen examples](https://github.com/rustwasm/wasm-bindgen/tree/main/examples)