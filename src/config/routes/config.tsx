import { HomeOutlined, LaptopOutlined, AppstoreOutlined, DropboxOutlined } from "@ant-design/icons";
import { Link } from "react-router-dom";
import AppPage from "../../pages/app";
import IndexPage from "../../pages/index/Index";
import LocalTerminalPage from "../../pages/local-terminal";
import EnvironmentListPage from "../../pages/environment";
import CreateEnvironmentPage from "../../pages/environment/create";
import React from "react";

export declare type Node = React.ReactNode | JSX.Element;
export declare type MenuAndRoute = {
    key: string, 
    path: string,
    menu?: boolean,
    element?: Node,
    icon?: Node,
    label?: Node | string,
    children?: MenuAndRoute[]
};


export const indexRoutes: MenuAndRoute[] = [
    {
        path: '/',
        element: <IndexPage/>,
        key: 'IndexPage',
        icon: <HomeOutlined/>,
        label: <Link to='/'>首页</Link>
    },
    {
        path: '/environment',
        key: 'EnvironmentPage',
        icon: <DropboxOutlined />,
        label: <Link to='/environment'>环境变量</Link>,
        children: [
            {
                path: '',
                key: 'EnvironmentListPage',
                menu: true,
                element: <EnvironmentListPage />,
                label: <Link to='/environment'>环境变量</Link>,
            },
            {
                path: 'create',
                key: 'CreateEnvironmentPage',
                element: <CreateEnvironmentPage />,
                label: <Link to='/environment/create'>新建环境</Link>,
            }
        ]
    },
    {
        path: '/app',
        element: <AppPage/>,
        key: 'AppPage',
        icon: <AppstoreOutlined />,
        label: <Link to='/app'>VApp</Link>
    },
    {
        path: 'terminal/local',
        element: <LocalTerminalPage/>,
        key: 'LocalTerminalPage',
        icon: <LaptopOutlined />,
        label: <Link to='/terminal/local'>本地终端</Link>,
    }
];