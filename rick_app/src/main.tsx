import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import {invoke} from "@tauri-apps/api";
import {listen} from "@tauri-apps/api/event";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);


listen('SYSTEM::APP_LOG', res => console.log(res)).then();

setTimeout(() => {
   invoke('test', {name: 'hello'}).then(resp => console.log(resp));
});
