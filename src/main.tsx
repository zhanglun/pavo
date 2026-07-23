import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App } from "./app/App";
import { applyInitialTheme } from "./features/theme/model/bootstrap";
import "./shared/styles/global.css";

applyInitialTheme();

createRoot(document.getElementById("app")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
