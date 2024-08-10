import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

const App = () => {
  const startGame = async () => {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke("start_game");
  };

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>
      <div className="row">
        <button
          type="button"
          onClick={(e) => {
            e.preventDefault();
            startGame();
          }}
        >
          Start Game
        </button>
      </div>
    </div>
  );
};

export default App;