import {Checkbox, Col, Form, Input, Row} from "antd";
import PathInput from "../../../component/path-input";


export default function NodeItem() {
    const labelCol = {xs: 8};
    return <>
        <Form.Item label={"执行目标"} required>
            <PathInput placeholder={"js文件路径或者package.json scripts名称"} filter={[{name: "*", extensions: ["js"]}]}/>
        </Form.Item>
        <Form.Item label={"运行目录"}>
            <PathInput placeholder={"可执行文件运行目录"} mode={'dir'}/>
        </Form.Item>
        <Form.Item label={"包管理器"}>
            <Input placeholder={"npm 或者 yarn 或者 cnpm"}/>
        </Form.Item>
        <Row>
            <Col span={12}>
                <Form.Item label={"管理员权限"} labelCol={labelCol}>
                    <Checkbox>管理员权限运行</Checkbox>
                </Form.Item>
            </Col>
            <Col span={12}>
                <Form.Item label={"单独页面"} labelCol={labelCol}>
                    <Checkbox>打开独立页面</Checkbox>
                </Form.Item>
            </Col>
        </Row>
        <Row>
            <Col span={12}>
                <Form.Item label={"程序交互"} labelCol={labelCol}>
                    <Checkbox>运行过程中可交互</Checkbox>
                </Form.Item>
            </Col>
            <Col span={12}>
                <Form.Item label={"依赖检测"} labelCol={labelCol}>
                    <Checkbox>运行前检测NodeJs环境</Checkbox>
                </Form.Item>
            </Col>
        </Row>
    </>;
}
