import {Form, Input} from "antd";


export default function WebSiteItem() {
    return <>
        <Form.Item label={"网站地址"} required>
            <Input placeholder={"https://xxxx.com"}/>
        </Form.Item>
    </>
}
