import { Button, Card, Form} from "antd";
import { useState } from "react";
import EnvironmentForm from "./environment-form";


function EnvironmentCreatePage() {
    
    const [loading, setLoading] = useState(false);
    const [form] = Form.useForm();
    const onSave = () => {
        console.log(form.getFieldsValue());
    };
    const extra = <Button.Group>
        <Button type="primary" onClick={() => onSave()}>保存</Button>
        <Button>取消</Button>
    </Button.Group>;
    return <Card title={"新建环境"} extra={extra} loading={loading} >
        <EnvironmentForm form={form} model={{}}/>
    </Card>
}

export default EnvironmentCreatePage;           