
import { ItemType, MenuItemGroupType } from 'antd/lib/menu/hooks/useItems';
import { useRoutes } from 'react-router'
import MainLayout from '../../layout/main-layout';
import { indexRoutes, MenuAndRoute } from './config';
export const AppRoute = () => useRoutes([
    {
        path: '/',
        element: <MainLayout />,
        children: indexRoutes
    }
]);



export declare type MenuItemType = ItemType & { path: string, children?: MenuItemType[] };

const getMenuList = () => {
    let menuList: MenuItemType[] = [];
    const each = (list: MenuItemType[], routes: MenuAndRoute[], parentPath: string) => {
        for (let it of routes) {
            if (it.menu === false) {
                continue;
            }
            let path = parentPath;
            if (it.path) {
                if (path.endsWith('/')) {
                    path += it.path;
                } else {
                    path += ('/' + it.path);
                }
            }
            // 格式化
            path = path.replace(/\/{2,}/, '/');
            let menu: MenuItemType = {
                key: it.key,
                label: it.label,
                path: path,
                icon: it.icon
            };
            list.push(menu);
            if (it.children?.length) {
                menu.children = [];
                each(menu.children!, it.children, menu.path);
            }
        }
    };
    each(menuList, indexRoutes, '');
    return menuList;
};

export const MenuList = getMenuList();
