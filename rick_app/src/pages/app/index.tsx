import {Button, Card, Col, Form, FormInstance, Input, List, message, Modal, Row} from "antd";
import './app.less';
import {useEffect, useState} from "react";
import {AppGroup} from "./app-model";
import {delAppGroup, listAppGroup, saveAppGroup, updateAppGroup} from "./app-request";
import {commonProcess, errorMessage} from "../../model";
import {DeleteOutlined, EditOutlined} from "@ant-design/icons";
import {dirDialogOpen, fileReadDialogOpen} from "../../utils/common";


interface AppGroupListProps {
    dataSource: AppGroup[];
    loading?: boolean;
    onSelected?: (value: AppGroup) => void;
    onEdit: (id: number, name: string) => void;
    onDelete: (id: number) => void;
}

function AppGroupList(props: AppGroupListProps) {
    const [selectedIndex, setSelectedIndex] = useState(0);
    const [hoverIndex, setHoverIndex] = useState(0);
    return <List bordered loading={props.loading} dataSource={props.dataSource} size={"small"}
                 renderItem={(value, index) => {
                     return <List.Item onClick={() => setSelectedIndex(index)}
                                       onMouseEnter={() => setHoverIndex(index)}
                                       onMouseLeave={() => setHoverIndex(-1)}
                                       actions={(index === selectedIndex || index === hoverIndex) &&
                                       index != 0 ? [<DeleteOutlined onClick={() => props.onDelete(value.id)}/>,
                                           <EditOutlined onClick={() => props.onEdit(value.id, value.name)}/>] : []}
                                       className={`rick-can-selected ${index === selectedIndex && 'rick-selected'}`}>
                         <span>{value.name}</span>
                     </List.Item>;
                 }}/>
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

function AppGroupForm(props: { form: FormInstance, name?: string }) {
    return <Form layout={"vertical"} form={props.form}>
        <Form.Item name={"name"} label={"分组名称"} required initialValue={props.name}>
            <Input/>
        </Form.Item>
    </Form>
}

export default function AppPage() {
    const [appGroupData, setAppGroupData] = useState([] as AppGroup[]);
    const [messageApi, messageContextHolder] = message.useMessage();
    const [modal, modalContextHolder] = Modal.useModal();
    const [loading, setLoading] = useState(false);
    const [form] = Form.useForm();
    const loadAppGroup = () => {
        setLoading(true);
        commonProcess(listAppGroup()).then((data) => {
            setAppGroupData([{id: 0, name: '全部', icon: ''}, ...data]);
            setLoading(false);
        }).catch(errorMessage(messageApi)).finally(() => {
            setLoading(false);
        });
    };

    const openSaveAppGroupForm = () => {
        const instance = modal.confirm({
            title: '新增分组',
            icon: null,
            content: <AppGroupForm form={form}/>,
            onOk: () => {
                const name = form.getFieldValue('name');
                setLoading(true);
                saveAppGroup(name)
                    .then(() => {
                        form.resetFields();
                        loadAppGroup();
                        messageApi.success('保存成功').then();
                    })
                    .catch((reason) => {
                        form.resetFields();
                        errorMessage(messageApi)(reason);
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
            content: <AppGroupForm form={form} name={name}/>,
            onOk: () => {
                const name = form.getFieldValue('name');
                setLoading(true);
                updateAppGroup(id, name)
                    .then(() => {
                        form.resetFields();
                        loadAppGroup();
                        messageApi.success('保存成功').then();
                    })
                    .catch((reason) => {
                        form.resetFields();
                        errorMessage(messageApi)(reason);
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
            onOk: () => {
                setLoading(true);
                delAppGroup(id)
                    .then(() => {
                        loadAppGroup();
                        messageApi.success('删除成功').then();
                    })
                    .catch((reason) => {
                        errorMessage(messageApi)(reason);
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
        loadAppGroup();
    }, []);
    const extraGroupNode = <><Button.Group>
        <Button onClick={() => openSaveAppGroupForm()}>新增</Button>
    </Button.Group></>;

    const selectFile = () => {
        let startTime = new Date().getTime();
        fileReadDialogOpen({multiple: true, title: '选择文件内容', need_content: true})
            .then(console.log).catch(console.log).finally(() => {
            console.log('耗时:', new Date().getTime() - startTime);
        });

    };

    const extraNode = <>
        <Button.Group>
            <Button onClick={() => selectFile()}>新增</Button>
        </Button.Group>
    </>;

    return <div className={"app_page"}>
        {messageContextHolder}
        {modalContextHolder}
        <Row gutter={8}>
            <Col className="group">
                <Card title={"分组"} extra={extraGroupNode}>
                    <AppGroupList dataSource={appGroupData} onEdit={openEditAppGroupForm} onDelete={openDeleteAppGroup}
                                  loading={loading}/>
                </Card>
            </Col>
            <Col flex={1}>
                <Card title={"App"} extra={extraNode}>
                    <AppList/>
                </Card>
            </Col>
        </Row>
    </div>
}
