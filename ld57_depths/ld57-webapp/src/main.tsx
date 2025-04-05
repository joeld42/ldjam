import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";

import init from "../ld57wasm/ld57_depths.js";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>
);

init().catch((error) => {
  if (
    !error.message.startsWith(
      "Using exceptions for control flow, don't mind me. This isn't actually an error!"
    )
  ) {
    throw error;
  }
});
