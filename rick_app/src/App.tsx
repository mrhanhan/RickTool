import "./App.css";
import {Button, ConfigProvider} from "antd";

import zhCN from 'antd/locale/zh_CN.js';
import {HashRouter} from "react-router-dom";
function App() {
  return <ConfigProvider locale={zhCN}>
    <Button>Hello</Button>
  </ConfigProvider>;
}

export default App;
