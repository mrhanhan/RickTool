import { HomeOutlined, LaptopOutlined, AppstoreOutlined, DropboxOutlined } from "@ant-design/icons";
import AppPage from "../../pages/app";
import React from "react";
import AppRuntimePage from "../../pages/app/runtime";

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
		element: <AppPage/>,
		key: 'IndexPage',
		icon: <HomeOutlined/>,
		label: <span>首页</span>
	},
	{
		path: '/app/runtime',
		key: 'AppRuntimePage',
		icon: <DropboxOutlined />,
		element: <AppRuntimePage/>,
		label: <span>执行环境</span>
	},
	{
		path: '/app',
		element: <AppPage/>,
		key: 'AppPage',
		icon: <AppstoreOutlined />,
		label: <span>VApp</span>
	},
	{
		path: 'terminal/local',
		element: <AppPage/>,
		key: 'LocalTerminalPage',
		icon: <LaptopOutlined />,
		label: <span>本地终端</span>,
	}
];
