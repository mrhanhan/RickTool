import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import {invoke} from "@tauri-apps/api";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

setTimeout(() => {
   invoke('greet', {name: 'hello'}).then();
   invoke('test', {name: 'hello', code: 100}).then();
});
