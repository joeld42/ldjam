import { useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import "../ld57wasm/ld57_depths";

function App() {
  const [count, setCount] = useState(0);

  return (
    <>
      {/* <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div> */}
      <div className="gamectr">
        <canvas
          id="game-canvas"
          className="inner"
          width="512"
          height="512"
        ></canvas>

        <div className="inner">
          <h2>Score: {count} </h2>

          <div className="card, inner">
            <button
              onClick={() => {
                setCount((count) => count + 1);
                console.log("hello");
              }}
            >
              count is {count}
            </button>
          </div>
        </div>
      </div>
    </>
  );
}

export default App;
