import {Button, Col, Input, Row, App as AntApp} from "antd";

import './app-start.less';
import {ReactNode, useRef, useState} from "react";
import ArgInput from "../../../component/arg-input";
import {AppArgs, AppStart as Start} from "../app-model";
import {ArgInputValue} from "../../../component/arg-input/arg-data";
import {getId} from "../../../utils/uid";


interface AppStartItemProps {
    /**
     * 按钮
     */
    buttons?: ReactNode[],
    value?: Partial<Start>,
    onChange?: (value: Partial<Start>) => void
}

function AppStartItem(props: AppStartItemProps) {
    const [inputMode, setInputMode] = useState(false);
    const [value, setValue] = useState(props.value || {});
    const onNameInput = (name: string) => {
        const val = {...value, name: name};
        setValue(val);
        props.onChange?.(val);
    }
    const onArgInput = (values: ArgInputValue[]) => {
        const val = {...value, args: values as AppArgs[]};
        setValue(val);
        props.onChange?.(val);
    }
    return <><div className={"app-start_item"}>
            <Row>
                <Col flex={1}>
                    {inputMode ? <Input placeholder={"请输入方案名称"} value={value.name} onInput={ e => onNameInput(e.currentTarget.value)} onKeyUp={e => {
                        if (e.key === 'Enter') {
                            setInputMode(false);
                        }
                    }}/> : <div onDoubleClick={() => setInputMode(true)}>{value.name}</div>}
                </Col>
                <Col>
                    <Button.Group size={"small"}>
                        {props.buttons}
                    </Button.Group>
                </Col>
            </Row>
            <Row gutter={8}>
                <Col className={"command-line-label"}>命令行:</Col>
                <Col flex={1}>
                    <ArgInput value={value.args} onChange={onArgInput}/>
                </Col>
            </Row>
        </div></>;
}

export interface AppStartProps {
    value?: Partial<Start[]>,
    onChange?: (value: Partial<Start>[]) => void
}

export default function AppStart(props: AppStartProps) {

    const {modal} = AntApp.useApp();
    const inputValueRef = useRef<string|null>(null);
    const onDelete = (index: number) => {
        if (!props.value) {
            return;
        }
        const array = [...props.value];
        array.splice(index, 1);
        props.onChange?.(array as Start[]);
    }

    const onAppend = (index: number) => {

        const append = () => {
            const array = props.value ? [...props.value] : [];
            let value: Partial<Start> = {id: getId(), name: inputValueRef.current || '启动方案:' + (array.length + 1)};
            if (array.length) {
                array.splice(index, 0, value as Start);
            } else {
                array.push(value as Start);
            }
            props.onChange?.(array as Start[]);
            instance.destroy();
            inputValueRef.current = null;
        }

        const instance = modal.confirm({
            title: '添加方案',
            icon: null,
            width: '400px',
            content: <div>
                请输入方案名称:
                <Input placeholder={"请输入启动方案名称 回车自动添加"} defaultValue={''}
                       onKeyDown={e => {
                           if (e.code === 'Enter') {
                               append();
                           }
                       }}
                       onInput={e => inputValueRef.current = e.currentTarget.value}/>
            </div>,
            onOk: () => {
                append();
            },
            onCancel: () => {
                instance.destroy();
            }
        });
    };

    const onChange = (index: number, value: Partial<Start>) => {
        if (!props.value) {
            return;
        }
        const array = [...props.value];
        array[index] = value as Start;
        props.onChange?.(array as Start[]);
        console.log(array);
    }
    return <div>
        {props.value?.map((it, index) =>
            <AppStartItem key={`app_start_item_${it?.id}`}
                          value={it}
                          onChange={value => onChange(index, value)}
                          buttons={[<Button key={`app_start_item_${it?.id}_add`} onClick={() => onAppend(index + 1)}>添加</Button>,
                              <Button key={`app_start_item_${it?.id}_rm`} onClick={() => onDelete(index)}>删除</Button>]}></AppStartItem>)}
        {
            (!props.value || !props.value.length) && <div>
                <Button block onClick={() => onAppend(0)}>添加启动方案</Button>
            </div>
        }
    </div>
}
