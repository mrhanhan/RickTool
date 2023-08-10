import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import {listen} from "@tauri-apps/api/event";
import {call} from "./utils/invoke";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);


listen('SYSTEM::APP_LOG', res => console.log(res)).then();

setTimeout(() => {
   call('test', {name: 'hello'})
       .then(resp => console.log(resp));
   call('list_app_group').then(resp => console.log(resp));
   call('save_app_group', {id: 0, name: '测试', icon: ''}).then(resp => console.log(resp)).then(() => {
       call('list_app_group').then(resp => console.log(resp));
   });
});
