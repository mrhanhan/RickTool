import { AutoComplete, Avatar, Col, Layout, Menu, Row } from "antd";
import { Outlet } from "react-router-dom";
import { useLocation, } from "react-router";
import { MenuItemType, MenuList } from "../../config/routes";

import './index.css';
function MainLayout() {

    const location = useLocation();
    let defaultSelectedKeys: string[] = [];
    const each = (menus: MenuItemType[]) => {
        for (let menu of menus) {
            if (menu.path === location.pathname) {
                defaultSelectedKeys.push(menu.key as string);
            }
            if (menu?.children?.length) {
                each(menu?.children!);
            }
        }
        console.log(location, defaultSelectedKeys, MenuList);
    }
    each(MenuList);
    return <Layout className="main-layout">
        <Layout.Sider className="main-layout_sider">
            <div className="main-layout_avatar">
                <Avatar src="/avatar.png" size={128}></Avatar>
                <div style={{color: "gray"}}>Rick Sanchez</div>
                <div style={{color: "gray"}}>Seraph 安全实验室</div>
            </div>
            <Menu items={MenuList} defaultSelectedKeys={defaultSelectedKeys}></Menu>
        </Layout.Sider>
        <Layout>
            <Layout.Header style={{paddingLeft: '0px'}}>
                <Row >
                    <Col flex="auto">
                        <AutoComplete placeholder="搜索..." style={{width: '30%'}}/>
                    </Col>
                    <Col flex="100px">

                    </Col>
                </Row>
            </Layout.Header>
            <Layout.Content style={{ padding: '8px', overflow: 'auto' }}>
                <Outlet />
            </Layout.Content>
        </Layout>
    </Layout>
}

export default MainLayout;
