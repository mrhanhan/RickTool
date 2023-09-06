import {Form, FormInstance, Input, Select, App as AntApp, Row, Col, Divider, Button, Checkbox} from "antd";
import {App} from "./app-model";
import {useEffect, useState} from "react";
import {commonProcess, errorMessage} from "../../model";
import {listAppGroup} from "./app-request";
import LogoInput from "../../component/logo-input";
import {AppType} from "./data";
import ExecuteItem from "./items/execute-item";
import JavaItem from "./items/java-item";
import PythonItem from "./items/python-item";
import NodeItem from "./items/nodejs-item";
import WebSiteItem from "./items/website-item";
import AppStart from "./items/app-start";


export interface AppFormProps {
    form?: FormInstance,
    model?: App,
}


export function AppForm(props: AppFormProps) {
    const [groupLoading, setGroupLoading] = useState(false);
    const [groupOptions, setGroupOptions] = useState([] as { label: string, value: number }[]);
    const {message} = AntApp.useApp();
    const loadGroup = () => {
        setGroupLoading(true);
        commonProcess(listAppGroup()).then(data => {
            setGroupOptions([{label: '无分组', value: 0}, ...data.map(it => ({label: it.name, value: it.id}))]);
        }).catch(errorMessage(message)).finally(() => {
            setGroupLoading(false);
        });
    };
    const targetType = Form.useWatch("target_type", props.form);

    useEffect(() => {
        loadGroup();
    }, []);
    return <div>
        <Form form={props.form} initialValues={{group_id: 0, target_type: 100, ...props.model}} autoComplete={"off"}>
            <Row gutter={16}>
                <Col xs={24} sm={24} md={12} className={"app_page-form_border"}>
                    <Divider dashed orientation={"left"}>基础信息</Divider>
                    <Row gutter={4}>
                        <Col span={8}>
                            <Form.Item label={""} name={"logo_path"} required>
                                <LogoInput/>
                            </Form.Item>
                        </Col>
                        <Col span={16}>
                            <Form.Item label={"分组"} name={"group_id"} required>
                                <Select options={groupOptions}
                                        onClick={loadGroup}
                                        loading={groupLoading}>
                                </Select>
                            </Form.Item>
                            <Form.Item label={"类型"} name={"target_type"} required>
                                <Select options={AppType}>
                                </Select>
                            </Form.Item>
                            <Form.Item label={"名称"} name={"name"} required>
                                <Input placeholder="请输入APP名称"/>
                            </Form.Item>
                        </Col>
                    </Row>
                    <Form.Item label={"备注"} name={"remark"}>
                        <Input.TextArea placeholder="请输入APP备注"/>
                    </Form.Item>
                </Col>
                <Col xs={24} sm={24} md={12}>
                    <Divider dashed orientation={"left"}>运行配置</Divider>
                    {targetType === 100 && <ExecuteItem/>}
                    {targetType === 200 && <JavaItem/>}
                    {targetType === 201 && <PythonItem/>}
                    {targetType === 202 && <NodeItem/>}
                    {targetType === 300 && <WebSiteItem/>}
                </Col>
            </Row>
            {targetType !== 300 && <>
                <Divider dashed orientation={"left"}>参数配置</Divider>
                <Row>
                    <Col span={24}>
                        <Form.Item label={"启动方案"} name={"start_vec"}>
                            <AppStart/>
                        </Form.Item>
                    </Col>
                </Row></>}
        </Form>
    </div>;
}
