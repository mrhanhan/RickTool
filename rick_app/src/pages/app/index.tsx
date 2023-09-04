import {Button, Card, Col, Form, FormInstance, Input, List, App as AntApp, Modal, Row} from "antd";
import './app.less';
import {useEffect, useState} from "react";
import {AppGroup} from "./app-model";
import {delAppGroup, listAppGroup, saveAppGroup, updateAppGroup} from "./app-request";
import {commonProcess, errorMessage} from "../../model";
import {DeleteOutlined, EditOutlined} from "@ant-design/icons";
import {AppForm} from "./app-form";


interface AppGroupListProps {
    onSelected?: (value: AppGroup) => void;
}

function AppGroupForm(props: { form: FormInstance, name?: string }) {
    return <Form layout={"vertical"} form={props.form}>
        <Form.Item name={"name"} label={"分组名称"} required initialValue={props.name}>
            <Input/>
        </Form.Item>
    </Form>
}

function AppGroupList(props: AppGroupListProps) {
    const [selectedIndex, setSelectedIndex] = useState(0);
    const [hoverIndex, setHoverIndex] = useState(0);
    const [appGroupData, setAppGroupData] = useState([] as AppGroup[]);
    const {message, modal} = AntApp.useApp();
    const [loading, setLoading] = useState(false);
    const [form] = Form.useForm();
    const openSaveAppGroupForm = () => {
        const instance = modal.confirm({
            title: '新增分组',
            icon: null,
            centered: true,
            content: <AppGroupForm form={form}/>,
            onOk: () => {
                const name = form.getFieldValue('name');
                setLoading(true);
                saveAppGroup(name)
                    .then(() => {
                        form.resetFields();
                        loadAppGroup();
                        message.success('保存成功').then();
                    })
                    .catch((reason) => {
                        form.resetFields();
                        errorMessage(message)(reason);
                        setLoading(false);
                    });
                instance.destroy();
            },
            onCancel: () => {
                instance.destroy();
            }
        });
    };
    const openEditAppGroupForm = (id: number, name: string) => {
        const instance = modal.confirm({
            title: '编辑分组',
            icon: null,
            centered: true,
            content: <AppGroupForm form={form} name={name}/>,
            onOk: () => {
                const name = form.getFieldValue('name');
                setLoading(true);
                updateAppGroup(id, name)
                    .then(() => {
                        form.resetFields();
                        loadAppGroup();
                        message.success('保存成功').then();
                    })
                    .catch((reason) => {
                        form.resetFields();
                        errorMessage(message)(reason);
                        setLoading(false);
                    });
                instance.destroy();
            },
            onCancel: () => {
                instance.destroy();
            }
        });
    };
    const openDeleteAppGroup = (id: number) => {
        const instance = modal.warning({
            title: '系统提示',
            content: '是否删除当前分组',
            centered: true,
            onOk: () => {
                setLoading(true);
                delAppGroup(id)
                    .then(() => {
                        loadAppGroup();
                        message.success('删除成功').then();
                    })
                    .catch((reason) => {
                        errorMessage(message)(reason);
                        setLoading(false);
                    });
                instance.destroy();
            },
            onCancel: () => {
                instance.destroy();
            }
        });
    };
    const loadAppGroup = () => {
        setLoading(true);
        commonProcess(listAppGroup()).then((data) => {
            setAppGroupData([{id: 0, name: '全部', icon: ''}, ...data]);
            setLoading(false);
        }).catch(errorMessage(message)).finally(() => {
            setLoading(false);
        });
    };
    const extraGroupNode = <><Button.Group>
        <Button onClick={() => openSaveAppGroupForm()}>新增</Button>
    </Button.Group></>;
    useEffect(() => {
        loadAppGroup();
    }, []);
    return <Card title={"分组"} extra={extraGroupNode}><List bordered loading={loading} dataSource={appGroupData}
                                                             size={"small"}
                                                             renderItem={(value, index) => {
                                                                 return <List.Item
                                                                     onClick={() => setSelectedIndex(index)}
                                                                     onMouseEnter={() => setHoverIndex(index)}
                                                                     onMouseLeave={() => setHoverIndex(-1)}
                                                                     actions={(index === selectedIndex || index === hoverIndex) &&
                                                                     index != 0 ? [<DeleteOutlined
                                                                         onClick={() => openDeleteAppGroup(value.id)}/>,
                                                                         <EditOutlined
                                                                             onClick={() => openEditAppGroupForm(value.id, value.name)}/>] : []}
                                                                     className={`rick-can-selected ${index === selectedIndex && 'rick-selected'}`}>
                                                                     <span>{value.name}</span>
                                                                 </List.Item>;
                                                             }}/></Card>
}

function AppList() {
    const data = [
        {
            title: 'Title 1',
        },
        {
            title: 'Title 2',
        },
        {
            title: 'Title 3',
        },
        {
            title: 'Title 4',
        },
    ];
    return <List dataSource={data}
                 grid={{gutter: 8, column: 4}}
                 renderItem={(item) => (
                     <List.Item>
                         <div>
                             {item.title}
                         </div>
                     </List.Item>
                 )}
    />;
}

export default function AppPage() {
    const {message, modal} = AntApp.useApp();
    const [form] = Form.useForm();
    const [loading, setLoading] = useState(false);
    const [groupId, setGroupId] = useState(0);
    const openCreateAppForm = () => {
        const instance = modal.confirm({
            title: '新增App',
            icon: null,
            width: '80%',
            content: <AppForm form={form}/>,
            onOk: () => {
                const name = form.getFieldValue('name');
                setLoading(true);
                saveAppGroup(name)
                    .then(() => {
                        form.resetFields();
                        message.success('保存成功').then();
                    })
                    .catch((reason) => {
                        form.resetFields();
                        errorMessage(message)(reason);
                        setLoading(false);
                    });
                instance.destroy();
            },
            onCancel: () => {
                instance.destroy();
            }
        });
    };

    const extraNode = <>
        <Button.Group>
            <Button onClick={openCreateAppForm}>新增</Button>
        </Button.Group>
    </>;

    return <div className={"app_page"}>
        <Row gutter={8}>
            <Col className="group">
                <AppGroupList onSelected={({id}) => setGroupId(id)}/>
            </Col>
            <Col flex={1}>
                <Card title={"App"} extra={extraNode} loading={loading}>
                    <AppList/>
                </Card>
            </Col>
        </Row>
    </div>
}
