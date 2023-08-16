import {AutoComplete, Avatar, Breadcrumb, Button, Col, Layout, Menu, Row} from "antd";
import {Link, Outlet} from "react-router-dom";
import { useLocation, } from "react-router";
import {getMenuPath, MenuList} from "../../config/routes";

import './index.css';
import React, {useState} from "react";
import {MenuFoldOutlined, MenuUnfoldOutlined} from "@ant-design/icons";
function MainLayout() {

    const location = useLocation();
    const [collapse, setCollapse] = useState(true);
    let items = getMenuPath(location.pathname);
    let defaultSelectedKeys: string[] = items.map(i => i.key as string);
    return <Layout className="main-layout">
        <Layout.Sider className={`main-layout_sider ${collapse && 'main-layout_collapsed'}`} collapsed={collapse}>
            <div className="main-layout_avatar">
                <Avatar src="/avatar.png" size={!collapse ? 128 : 64}></Avatar>
                <div style={{color: "gray", display: collapse ? 'none' : 'block'}}>Rick Sanchez</div>
                <div style={{color: "gray", display: collapse ? 'none' : 'block'}}>Seraph 安全实验室</div>
            </div>
            <Menu items={MenuList.map(it => {
                let data = {...it};
                data.label = <Link to={it.path}>{it.label}</Link>
                return data;
            })} defaultSelectedKeys={defaultSelectedKeys}></Menu>
        </Layout.Sider>
        <Layout>
            <Layout.Header style={{paddingLeft: '0px'}}>
                <Row >
                    <Col>
                        <Button icon={collapse ? <MenuUnfoldOutlined /> : <MenuFoldOutlined />} onClick={() => setCollapse(!collapse)}>
                            {collapse ? '展开' : '合并'}
                        </Button>
                    </Col>
                    <Col flex="auto">
                        <AutoComplete placeholder="搜索..." style={{width: '30%'}}/>
                    </Col>
                    <Col flex="100px">

                    </Col>
                </Row>
            </Layout.Header>
            <Layout.Content style={{ padding: '8px', overflow: 'auto' }}>
                <Breadcrumb items={items.map(it => ({key: it.key, path: it.path, title: <span>{it.label}</span>}))}/>
                <Outlet />
            </Layout.Content>
        </Layout>
    </Layout>
}

export default MainLayout;
