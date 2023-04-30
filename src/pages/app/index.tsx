import { PlusCircleOutlined, DeleteOutlined, EditOutlined } from "@ant-design/icons";
import { Button, Card, Col, Form, Input, Menu, message, Modal, Row } from "antd";
import { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { ContextMenu, ContextMenuItem, ContextMenuTrigger } from "../../components/context-menu-panel";
import IconFont from "../../components/iconfont";
import { updateVAppGroup, addVAppGroup, delVAppGroup, selectData } from "../../store/configSlice";
import { VApp, VAppGroup } from "../../store/vapp";
import { copy } from "../../utils/copy";
import { genId } from "../../utils/id";
import VAppForm, { createAppFormContext } from "./app-form";
import AppItem from "./app-item";
import { runApp } from "./app-run";
import './index.css';

function AppPage() {

    const [loading, setLoading] = useState(false);
    const data = useSelector(selectData);
    const [selectedKey, setSelectedKey] = useState('');
    useEffect(() => {
        if (data.groups.length > 0 && !selectedKey) {
            setSelectedKey(data.groups[0].id);
        }
    }, [data]);
    const [groupForm] = Form.useForm();
    const dispatch = useDispatch();
    const onAddGroup = () => {
        setLoading(true);
        // 打开弹窗
        Modal.confirm({
            title: '新增VApp分组',
            okText: '保存',
            cancelText: '取消',
            content: <div>
                <Form form={groupForm} size="small" layout="vertical">
                    <Form.Item name="groupName" required={true} preserve={false}>
                        <Input placeholder="分组名称" />
                    </Form.Item>
                </Form>
            </div>,
            onOk: () => {
                const groupName = groupForm.getFieldValue("groupName");
                if (groupName == null) {
                    message.warning('请输入分组名称').then();
                } else {
                    dispatch(addVAppGroup({ name: groupName, id: genId() }));
                    message.success('添加成功').then();
                }
                setLoading(false);
            },
            onCancel: () => {
                setLoading(false)
            }
        });
    };
    const onEditGroup = (group: VAppGroup) => {
        setLoading(true);
        // 打开弹窗
        Modal.confirm({
            title: '编辑VApp分组',
            okText: '保存',
            cancelText: '取消',
            content: <div>
                <Form form={groupForm} size="small" layout="vertical">
                    <Form.Item name="groupName" preserve={false} required={true} initialValue={group.name}>
                        <Input placeholder="分组名称" />
                    </Form.Item>
                </Form>
            </div>,
            onOk: () => {
                const groupName = groupForm.getFieldValue("groupName");
                if (groupName == null) {
                    message.warning('请输入分组名称').then();
                } else {
                    dispatch(updateVAppGroup({ ...group, name: groupName }));
                    message.success('编辑成功').then();
                }
                setLoading(false);
            },
            onCancel: () => {
                setLoading(false);
            }
        });
    };
    const onDelGroup = (group: VAppGroup) => {
        setLoading(true);
        dispatch(delVAppGroup(group.id));
        setLoading(false);
        message.success('删除成功').then();
    };

    const onAddVApp = () => {
        const form = createAppFormContext();
        const onSubmit = (app: VApp) => {
            data.groups.forEach((it) => {
                if (it.id === selectedKey) {
                    const group = copy(it);
                    app.id = genId();
                    if (!group.app) {
                        group.app = [app];
                    } else {
                        group.app.push(app);
                    }
                    dispatch(updateVAppGroup(group));
                }
            });
        }

        Modal.confirm({
            title: '新增vApp',
            width: '640px',
            content: <div>
                <VAppForm form={form} onSubmit={onSubmit} />
            </div>,
            okText: '保存',
            cancelText: '取消',
            onOk: () => {
                form.submit();
            },
            onCancel: () => {
                form.cancel();
            }
        });
    };

    const onEditVApp = (editApp: VApp, gruopId: string)=> {
        const form = createAppFormContext();
        const onSubmit = (app: VApp) => {
            data.groups.forEach((it) => {
                if (it.id === gruopId) {
                    const group = copy(it);
                    group.app = group.app ? group.app.map(a => {
                        if (a.id === editApp.id) {
                            return app;
                        }
                        return a;
                    }) : [app];
                    dispatch(updateVAppGroup(group));
                }
            });
        }
        Modal.confirm({
            title: '编辑vApp',
            width: '640px',
            content: <div>
                <VAppForm form={form} defaultValue={editApp} onSubmit={onSubmit} />
            </div>,
            okText: '保存',
            cancelText: '取消',
            onOk: () => {
                form.submit();
            },
            onCancel: () => {
                form.cancel();
            }
        });
    }

    const onDelVApp = (appId: string, groupId: string) => {
        data.groups.forEach((it) => {
            if (it.id === groupId) {
                const group = copy(it);
                if (group.app){
                    group.app = group.app.filter(a => a.id !== appId );
                }
                dispatch(updateVAppGroup(group));
            }
        });
    };
    
    const onRun = (item:{app: VApp, group: VAppGroup}) => {
        runApp(item.app);
    };
    return <div className="app-page">
        <Row gutter={8}>
            <Col flex="300px">
                <Card>
                    <Button icon={<PlusCircleOutlined />}
                        block={true}
                        loading={loading}
                        style={{ marginBottom: '16px' }}
                        onClick={onAddGroup}>
                        添加分组
                    </Button>
                    <Menu
                        selectedKeys={[selectedKey]}
                        onSelect={({ key }) => {
                            setSelectedKey(key);
                        }} items={data.groups.map((t) => ({
                            key: t.id,
                            label: <Row>
                                <Col flex="auto">{t.name}</Col>
                                <Col>
                                    <Button.Group size="small" className="app-page_item_button">
                                        <Button icon={<EditOutlined />} loading={loading} onClick={() => onEditGroup(t)}></Button>
                                        <Button icon={<DeleteOutlined />} loading={loading} onClick={() => onDelGroup(t)}></Button>
                                    </Button.Group>
                                </Col>
                            </Row>

                        }))}>
                    </Menu>
                </Card>
            </Col>
            <Col flex="1 1 0">
                <Card title={
                    <div>
                        <Button icon={<PlusCircleOutlined />} disabled={!setSelectedKey} onClick={onAddVApp}>添加 vApp</Button>
                    </div>
                }><div style={{ display: 'flex', flexWrap: 'wrap' }}>
                        {
                            data.groups.filter(it => it.id === selectedKey)
                                .flatMap(it => (it.app || []).map(app => ({ app, group: it })))
                                .map((item, index) => <ContextMenuTrigger key={`app-item_${index}`}
                                    contextMenuId="V_APP_CONTEXT_MENU" data={item} clickTrigger={true}>
                                    <AppItem app={item.app} group={item.group} />
                                </ContextMenuTrigger>
                                )
                        }
                    </div>
                </Card>
            </Col>
        </Row>
        <ContextMenu id="V_APP_CONTEXT_MENU" width={150}>
            <ContextMenuItem icon={<IconFont icon="yunhang"/>} onClick={(item: {app: VApp, group: VAppGroup}) => onRun(item)}>运行</ContextMenuItem>
            <ContextMenuItem icon={<IconFont icon="bianji"/>} onClick={(item: {app: VApp, group: VAppGroup}) => onEditVApp(item.app, item.group.id)}>编辑</ContextMenuItem>
            <ContextMenuItem icon={<IconFont icon="delete"/>} onClick={(item: {app: VApp, group: VAppGroup}) => {onDelVApp(item.app.id, item.group.id);}}>删除</ContextMenuItem>
            <ContextMenuItem icon={<IconFont icon="open"/>} onClick={console.log}>资源管理器打开</ContextMenuItem>
        </ContextMenu>
    </div>
}

export default AppPage;