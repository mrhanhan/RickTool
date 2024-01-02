import {
    Button,
    Card,
    Col,
    Form,
    FormInstance,
    Input,
    List,
    App as AntApp,
    Modal,
    Row,
    Image,
    Avatar,
    Divider
} from "antd";
import './app.less';
import {useEffect, useRef, useState} from "react";
import {App, AppGroup} from "./app-model";
import {
    delApp,
    delAppGroup,
    detailApp,
    listApp,
    listAppGroup,
    saveApp,
    saveAppGroup, updateApp,
    updateAppGroup
} from "./app-request";
import {commonProcess, errorMessage, getErrorMessage} from "../../model";
import {DeleteOutlined, EditOutlined} from "@ant-design/icons";
import {AppForm} from "./app-form";
import {appToForm, numberToBase64Img, processForm} from "./data";
import {DEFAULT_LOGO_URL} from "../../component/logo-input";
import ContextMenu from "../../component/context-menu";
import ContextMenuItem from "../../component/context-menu/context-menu-item";


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
        const instance = modal.confirm({
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
    return <Card title={"分组"} extra={extraGroupNode}>
        <List bordered loading={loading} dataSource={appGroupData}
              size={"small"}
              renderItem={(value, index) => {
                  return <List.Item
                      onClick={() => {
                          setSelectedIndex(index);
                          props.onSelected?.(value);
                      }}
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

declare interface AppListProps {
    dataSource: App[],
    onEdit: (app: App) => void
    onDel: (app: App) => void
    onRun: (app: App) => void
    onRunAs: (app: App) => void
}

function AppList(props: AppListProps) {

    const contextMenuRender = (app: App) => <div className={"app-context-menu-layout"}>
        <List size={"small"}>
            <List.Item onClick={() => props.onRun(app)}>运行</List.Item>
            <List.Item onClick={() => props.onRun(app)}>管理员运行</List.Item>
            <List.Item onClick={() => props.onEdit(app)}>编辑</List.Item>
            <List.Item onClick={() => props.onDel(app)}>删除</List.Item>
        </List>
    </div>;

    return <ContextMenu onRender={contextMenuRender}>
        <List dataSource={props.dataSource}
              rowKey={"id"}
              grid={{gutter: 8, column: 12}}
              renderItem={(item) => (
                  <List.Item>
                      <ContextMenuItem bind={item}>
                          <div style={{textAlign: 'center'}}>
                              <Avatar src={item.logo?.length ? numberToBase64Img(item.logo!) : DEFAULT_LOGO_URL}
                                      size={48}/>
                              <br/>
                              <div>{item.name}</div>
                          </div>
                      </ContextMenuItem>
                  </List.Item>
              )}
        /></ContextMenu>;
}

export default function AppPage() {
    const {message, modal} = AntApp.useApp();
    const [form] = Form.useForm();
    const groupIdRef = useRef(0);
    const [loading, setLoading] = useState(false);
    const [dataSource, setDataSource] = useState([] as App[]);
    const onLoad = (groupId: number) => {
        setLoading(true);
        commonProcess(listApp({group_id: groupId})).then((data) => {
            setDataSource(data);
            console.log(data);
            setLoading(false);
        }).catch(reason => {
            message.error('获取应用失败:' + getErrorMessage(reason)).then();
            setLoading(false);
        });
    }
    const openCreateAppForm = () => {
        const instance = modal.confirm({
            title: '新增App',
            icon: null,
            width: '80%',
            content: <AppForm form={form} model={{group_id: groupIdRef.current}}/>,
            onOk: () => {
                setLoading(true);
                let model = processForm(form.getFieldsValue());
                commonProcess(saveApp(model)).then(app => {
                    message.success("保存App成功").then();
                    setLoading(false);
                    onLoad(groupIdRef.current);
                    instance.destroy();
                }).catch(reason => {
                    message.error('保存失败:' + getErrorMessage(reason)).then();
                    setLoading(false);
                });
            },
            onCancel: () => {
                instance.destroy();
            }
        });
    };

    const openEditAppForm = (id: number) => {
        commonProcess(detailApp(id)).then(data => {
            let model = appToForm(data);
            const instance = modal.confirm({
                title: '编辑App',
                icon: null,
                width: '80%',
                content: <AppForm form={form} model={model}/>,
                onOk: () => {
                    setLoading(true);
                    let model = processForm(form.getFieldsValue());
                    model.id = data.id;
                    model.create_time = data.create_time;
                    commonProcess(updateApp(model)).then(app => {
                        message.success("保存App成功").then();
                        setLoading(false);
                        onLoad(groupIdRef.current);
                        instance.destroy();
                    }).catch(reason => {
                        message.error('保存失败:' + getErrorMessage(reason)).then();
                        setLoading(false);
                    });
                },
                onCancel: () => {
                    instance.destroy();
                }
            });
        }).catch((reason) => {
            message.error(getErrorMessage(reason)).then();
        });
    };

    const openDeleteApp = (id: number) => {
        const instance = modal.confirm({
            title: '系统提示',
            content: '是否删除当前App',
            onOk: () => {
                setLoading(true);
                delApp(id)
                    .then(() => {
                        onLoad(groupIdRef.current);
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
    useEffect(() => {
        onLoad(groupIdRef.current);
    }, []);

    const extraNode = <>
        <Button.Group>
            <Button onClick={() => openCreateAppForm()}>新增</Button>
        </Button.Group>
    </>;

    return <div className={"app_page"}>
        <Row gutter={8}>
            <Col className="group">
                <AppGroupList onSelected={({id}) => {
                    groupIdRef.current = id;
                    onLoad(id);
                }}/>
            </Col>
            <Col flex={1}>
                <Card title={"App"} extra={extraNode} loading={loading}>
                    <AppList dataSource={dataSource}
                             onEdit={app => openEditAppForm(app.id)}
                             onRun={app => {
                                 
                             }}
                             onRunAs={app => {}}
                             onDel={app => openDeleteApp(app.id)}
                    />
                </Card>
            </Col>
        </Row>
    </div>
}
