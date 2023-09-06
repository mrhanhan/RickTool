import {
    Alert,
    Button,
    Card,
    Checkbox,
    Col,
    Form,
    FormInstance, FormRule,
    Input,
    List,
    message,
    Modal,
    Row, Spin,
    Tag
} from "antd";
import {useEffect, useState} from "react";
import {
    AppRuntime, AppRuntimeItem,
    AppRuntimeItemForm,
    AppRuntimeItemInputProps, deleteAppRuntime, detailAppRuntime,
    listAppRuntime,
    saveAppRuntime, updateAppRuntime
} from "./app-runtime-request";
import {commonProcess, errorMessage} from "../../../model";
import {DeleteOutlined, EditOutlined, FileOutlined, PlusSquareOutlined} from "@ant-design/icons";
import {dirDialogOpen} from "../../../utils/common";


function AppRuntimeItemInput(props: AppRuntimeItemInputProps) {

    const [selectLoading, setSelectLoading] = useState(false);
    const [messageApi, messageContextHolder] = message.useMessage();
    const onAddNewItem = () => {
        if (!!props.value) {
            props.onChange?.([...props.value!, {code: '', value: ''}]);
        } else {
            props.onChange?.([{code: '', value: ''}]);
        }
    };
    const onDelItem = (index: number) => {
        if (!!props.value) {
            let data = [...props.value!];
            data.splice(index, 1);
            props.onChange?.(data);
        }
    }
    const onInput = (field: keyof AppRuntimeItem, index: number) => {
      return (value: string) => {
          let data = [...props.value!];
          let row = data[index];
          row[field] =  value as any;
          console.log(data);
          props.onChange?.(data);
      };
    };
    const onSelectDirPath = (index: number) => {
        setSelectLoading(true);
        commonProcess(dirDialogOpen({multiple: true})).then(res => {
            let value = res.map(it => it.path).join(';');
            onInput('value', index)(value);
        }).catch(errorMessage(messageApi)).finally(() => {
            setSelectLoading(false);
        });
    };
    return <>
        {messageContextHolder}
        <Spin spinning={selectLoading}>
        {
            props.value?.map((it, index) => {
                return <Row className="runtime_item_input_wrapper" gutter={8} key={`runtime_item_input_wrapper_${index}`}>
                    <Col span={8}>
                        <Input placeholder={"输入环境变量名称: PATH、JAVA_HOST"} defaultValue={it.code}
                               onInput={e => onInput("code", index)(e.currentTarget.value)}
                               autoComplete={"off"}/>
                    </Col>
                    <Col span={16}>
                        <Input placeholder={"输入环境变量值"} autoComplete={"off"}
                                                                       value={it.value}
                               onInput={e => onInput("value", index)(e.currentTarget.value)}
                                     addonAfter={<><FileOutlined onClick={() => onSelectDirPath(index)} />&nbsp;&nbsp;<DeleteOutlined onClick={() => onDelItem(index)}/></>}/>
                    </Col>
                </Row>
            })
        }
        </Spin>
        <Button loading={selectLoading} icon={<PlusSquareOutlined />} block type={"dashed"} onClick={() => onAddNewItem()}>新增环境变量</Button>
    </>
}

function AppRuntimeForm(props: { form: FormInstance, model?: AppRuntime }) {

    const {model, form} = props;
    const rules: Record<string, FormRule[]> = {
        name: [{required: true}],
        description: [{required: true}],
        include_system: [{required: true}],
        items: [{validator: (_, value) => {
            let val = value as AppRuntimeItemForm[] | undefined;
            if (!val?.filter(it => !!it.code && !!it.value).length) {
                return Promise.reject('请输入环境变量');
            }
            return Promise.resolve(val);
        }}],
    };
    const extra = <Alert type={"info"} message={<>
        环境变量输入示例:<br/>
        <Tag>FLAGS=exec_<span style={{color: "green"}}>{'{$platform}'}</span>.exe</Tag>
        <Tag>msg=hello <span style={{color: "green"}}>$name</span></Tag>&nbsp;
        <Tag>HOME=/<span style={{color: "green"}}>$name</span>/bin</Tag>&nbsp;
    </>}/>;
    return <div style={{minHeight: "200px"}} ><Form layout={"vertical"} initialValues={props.model} form={form} preserve={false}>
        <Row gutter={8} >
            <Col span={12}>
                <Form.Item name={"name"} label={"执行环境"} required initialValue={model?.name} rules={rules.name}>
                    <Input placeholder="执行环境名称"/>
                </Form.Item>
                <Form.Item name={"include_system"} label={"系统环境"} required
                           valuePropName="checked"
                           initialValue={model?.include_system} rules={rules.include_system}>
                    <Checkbox >继承系统环境</Checkbox>
                </Form.Item>
                <Form.Item name={"description"} label={"备注"} initialValue={model?.description} rules={rules.description}>
                    <Input.TextArea placeholder="当前环境的备注" showCount maxLength={255}/>
                </Form.Item>

            </Col>
            <Col span={12}>
                <Form.Item name={"items"} label={"环境变量"} required initialValue={model?.items} extra={extra}
                           rules={rules.items}>
                    <AppRuntimeItemInput/>
                </Form.Item>
            </Col>
        </Row>
    </Form></div>;
}

export default function AppRuntimePage() {
    const [messageApi, messageContextHolder] = message.useMessage();
    const [modal, modalContextHolder] = Modal.useModal();
    const [loading, setLoading] = useState(false);
    const [form] = Form.useForm();
    const [dataSource, setDataSource] = useState([] as AppRuntime[]);

    const loadAppRuntimeList = () => {
        setLoading(true);
        commonProcess(listAppRuntime()).then(data => {
            setDataSource(data);
        }).catch(errorMessage(messageApi)).finally(() => {
            setLoading(false);
        });
    };
    const openSaveAppRuntimeForm = () => {
        const instance = modal.confirm({
            title: '新增执行环境',
            icon: null,
            width: '80%',
            content: <AppRuntimeForm form={form}/>,
            onOk: async () => {
                try {
                    const result = await form.validateFields();
                    console.log('Result:', result);
                    setLoading(true);
                    saveAppRuntime(result)
                       .then(() => {
                           form.resetFields();
                           loadAppRuntimeList();
                           messageApi.success('保存成功').then();
                           setLoading(false);
                       })
                       .catch((reason) => {
                           form.resetFields();
                           errorMessage(messageApi)(reason);
                           setLoading(false);
                       });
                } catch (reason) {
                    throw '';
                }
            },
            onCancel: () => {
                instance.destroy();
            }
        });
    };

    const openUpdateAppRuntimeForm = (id: number) => {
        setLoading(true);
        commonProcess(detailAppRuntime(id)).then(model => {
            const instance = modal.confirm({
                title: '修改执行环境',
                icon: null,
                width: '80%',
                content: <AppRuntimeForm form={form} model={model}/>,
                onOk: async () => {
                    try {
                        const result = await form.validateFields();
                        console.log('Result:', result);
                        setLoading(true);
                        updateAppRuntime(result, id)
                            .then(() => {
                                form.resetFields();
                                loadAppRuntimeList();
                                messageApi.success('保存成功').then();
                            })
                            .catch((reason) => {
                                form.resetFields();
                                errorMessage(messageApi)(reason);
                                setLoading(false);
                            });
                    } catch (reason) {
                        throw '';
                    }
                },
                onCancel: () => {
                    instance.destroy();
                }
            });
        }).catch(errorMessage(messageApi)).finally(() => {
            setLoading(false);
        })
    };
    const onDeleteAppRuntime = (id: number) => {
        const instance = modal.confirm({
            title: '系统提示',
            content: '是否删除当前运行环境',
            onOk: () => {
                setLoading(true);
                deleteAppRuntime(id)
                    .then(() => {
                        loadAppRuntimeList();
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
        loadAppRuntimeList();
    }, []);
    const extraNode = <><Button.Group>
        <Button onClick={() => openSaveAppRuntimeForm()}>新增</Button>
    </Button.Group></>;
    const renderAppRuntimeItem = (_value: AppRuntime, _index: number) => {
        return <List.Item actions={[<DeleteOutlined onClick={() => onDeleteAppRuntime(_value.id)} />,
            <EditOutlined onClick={() => openUpdateAppRuntimeForm(_value.id)}/>]}>
            <List.Item.Meta title={<>{_value.name}&nbsp;{!!_value.include_system && <Tag color={"success"}>继承系统环境</Tag>}</>}
            description={<>{_value.description}</>}/>
        </List.Item>
    }
    return <div className={"app-runtime-page"}>
        <Card title={"执行环境"} extra={extraNode} loading={loading}>
            <List dataSource={dataSource} renderItem={renderAppRuntimeItem}/>
        </Card>
        {modalContextHolder}
        {messageContextHolder}
    </div>
}
