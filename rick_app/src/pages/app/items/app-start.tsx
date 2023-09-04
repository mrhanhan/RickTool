import {Col, Input, Row} from "antd";

import './app-start.less';
import {useState} from "react";
import Index from "../../../component/arg-input";

function AppStartItem() {
    const [value, setValue] = useState('app ');
    const onInput = (value: string) => {
        if (!value.startsWith('app ')) {
            setValue('app ')
        } else {
            setValue(value);
        }
    }
    return <><div className={"app-start_item"}>
            <div>方案名称</div>
            <Row gutter={8}>
                <Col className={"command-line-label"}>命令行:</Col>
                <Col flex={1}><Input value={value} onInput={(e) => onInput(e.currentTarget.value)}/></Col>
            </Row>
        </div></>;
}

export default function AppStart() {
    return <div>
        <Index/>
        <AppStartItem/>
        <AppStartItem/>
        <AppStartItem/>
    </div>
}
