import {Checkbox, Col, Form, Row} from "antd";
import PathInput from "../../../component/path-input";


export default function PythonItem() {
    const labelCol = {xs: 8};
    return <>
        <Form.Item label={"Python脚本"} required name={"target"}>
            <PathInput placeholder={"执行程序路径"} filter={[{name: "*", extensions: ["py"]}]}/>
        </Form.Item>
        <Form.Item label={"运行目录"} name={"ext.run_dir"}>
            <PathInput placeholder={"可执行文件运行目录"} mode={'dir'}/>
        </Form.Item>
        <Form.Item label={"依赖文件"} name={"ext.python_pip_requirement"}>
            <PathInput placeholder={"PIP依赖文件路径"}/>
        </Form.Item>
        <Row>
            <Col span={12}>
                <Form.Item label={"管理员权限"} labelCol={labelCol} valuePropName={"checked"} name={"ext.sudo"}>
                    <Checkbox>管理员权限运行</Checkbox>
                </Form.Item>
            </Col>
            <Col span={12}>
                <Form.Item label={"单独页面"} labelCol={labelCol} valuePropName={"checked"} name={"ext.page"}>
                    <Checkbox>打开独立页面</Checkbox>
                </Form.Item>
            </Col>
        </Row>
        <Row>
            <Col span={12}>
                <Form.Item label={"程序交互"} labelCol={labelCol} valuePropName={"checked"} name={"ext.interact"}>
                    <Checkbox>运行过程中可交互</Checkbox>
                </Form.Item>
            </Col>
            <Col span={12}>
                <Form.Item label={"依赖检测"} labelCol={labelCol} valuePropName={"checked"} name={"ext.python_pip"}>
                    <Checkbox>运行前检测PIP环境</Checkbox>
                </Form.Item>
            </Col>
        </Row>
    </>;
}
