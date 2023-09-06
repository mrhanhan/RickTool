import {ArgInputValue, ArgType, ArgTypeColorMap, ArgTypeNameMap} from "./arg-data";
import {Checkbox, Col, Input, InputRef, List, Row, Tag} from "antd";
import PathInput from "../path-input";
import {useEffect, useRef, useState} from "react";

export enum ArgConfigMode {
    EDIT = 1,
    SHOW = 2
}

export interface ArgConfigProps {
    value?: ArgInputValue;
    mode?: ArgConfigMode,
    onChange?: (value: ArgInputValue) => void;
}

export default function ArgConfig(props: ArgConfigProps) {
    const [model, setModel] = useState<ArgInputValue>({...(props.value || {}) as ArgInputValue});
    const defaultValueInputRef = useRef<InputRef>(null);
    const onUpdate = (value: Partial<ArgInputValue>) => {
        setModel({...model, ...value});
        props.onChange?.({...model, ...value})
    };
    return <div>
        <Row gutter={2}>
            <Col span={4}>类型</Col>
            <Col span={8}><Tag color={ArgTypeColorMap[model.ty]}>{ArgTypeNameMap[model.ty]}</Tag></Col>
            <Col span={4}>名称</Col>
            <Col span={8}>
                <Input size={"small"} ref={model.ty === ArgType.FILE ? null : defaultValueInputRef}  value={model.name} onInput={(e) => {
                    onUpdate({name: e.currentTarget.value});
                }}/>
            </Col>
        </Row>
        <Row gutter={2}>
            <Col span={4}>默认值</Col>
            <Col span={20}>
                {model.ty === ArgType.FILE ?
                    <PathInput size={"small"} value={model.default_value} filter={[]} mode={"file"} onChange={value => {
                        onUpdate({default_value: value as string});
                    }}/> :
                    <Input ref={defaultValueInputRef} size={"small"} value={model.default_value} onInput={(e) => {
                        onUpdate({default_value: e.currentTarget.value});
                    }}/>}
            </Col>
        </Row>
        <Row gutter={2}>
            <Col span={4}>标记</Col>
            <Col span={20}>
                <Checkbox defaultChecked={model.optional === 1} onChange={e => {
                    onUpdate({optional: e.target.checked ? 1 : 0});
                }}>可选参数</Checkbox>
                <Checkbox defaultChecked={model.multiple === 1} onChange={e => {
                    onUpdate({multiple: e.target.checked ? 1 : 0});
                }}>多选参数</Checkbox>
            </Col>
        </Row>
        {model.ty === ArgType.SELECT && <Row gutter={2}>
            <Col span={4}>配置</Col>
            <Col span={20}>
                <List dataSource={[]}/>
            </Col>
        </Row>}
        <Row gutter={2}>
            <Col span={4}>描述</Col>
            <Col span={20}>
                <Input.TextArea value={model.remark} onInput={(e) => {
                    onUpdate({remark: e.currentTarget.value});
                }} maxLength={2} autoSize={{minRows: 2, maxRows: 2}}/>
            </Col>
        </Row>
    </div>
}
