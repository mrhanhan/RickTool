import "./App.css";
import { ConfigProvider, theme} from "antd";

import zhCN from 'antd/locale/zh_CN.js';
import {HashRouter} from "react-router-dom";
import {AppRoute} from "./config/routes";

function App() {
    return <ConfigProvider locale={zhCN} theme={{algorithm: theme.darkAlgorithm}} componentSize={'small'} >
        <HashRouter>
            <AppRoute/>
        </HashRouter>
    </ConfigProvider>;
}

export default App;
