import ReactDOM from 'react-dom/client';
import { HashRouter } from 'react-router-dom';
import { AppRoute } from './config/routes';
import 'antd/dist/antd.dark.css';
import './styles.css';
import { initApp } from './config/init';
import { Provider } from 'react-redux';
import store from './store';
import { ConfigProvider, message } from 'antd';
import zhCN from 'antd/lib/locale/zh_CN';
import { StrictMode } from 'react';
import { api, api_json } from './utils/api';


initApp();


function Main() {
  api('hello', 'data').then((data) => {
    message.info('调用完成' + data);
  });
  api_json('hello_json', {data: 'JSON 数据'}).then((data) => {
    message.info('调用完成' + data.data);
  });
  
  return <StrictMode><Provider store={store}>
        <HashRouter>
          <ConfigProvider locale={zhCN}>
            <AppRoute />          
          </ConfigProvider>
        </HashRouter>
      </Provider></StrictMode>;
}

ReactDOM.createRoot(document.getElementById('root')!).render(<Main />);