import {Checkbox, Col, Form, Row} from "antd";
import PathInput from "../../../component/path-input";


export default function JavaItem() {
    const labelCol = {xs: 8};
    return <>
        <Form.Item label={"Jar包路径"} required>
            <PathInput placeholder={"执行程序路径"} filter={[{name: "*", extensions: ["jar", "class"]}]}/>
        </Form.Item>
        <Form.Item label={"运行目录"}>
            <PathInput placeholder={"可执行文件运行目录"} mode={'dir'}/>
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
            </Col>
        </Row>
    </>;
}
