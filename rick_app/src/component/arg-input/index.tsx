import './arg-input.less';
import {KeyboardEvent, useEffect, useRef, useState} from "react";
import {AutoComplete, App as AntApp, Col, Row, Tag} from "antd";
import TextCircle from "../text-circle";
import {BaseSelectRef} from "rc-select";
import BlockCircle from "../block-circle";
import {ArgInputValue, ArgTypeColorMap, ArgType} from "./arg-data";
import ArgConfig from "./arg-config";

const ArgInputOptions = [
    {label: <div><Tag color={"blue"}>固定参数</Tag> 此参数在执行命令的过程中属于固定位置参数,
            例如:
            <TextCircle>pip install <Tag color={"blue"}>-r</Tag></TextCircle>,
            <TextCircle>apt <Tag color={"blue"}>search</Tag></TextCircle></div>, value: 1},
    {label: <div>
            <Tag color={"green"}>文件参数</Tag> 此参数在执行命令前可输入文件路径或者选择文件路径，选择后会转换为固定参数。
            例如：<TextCircle>pip install -r <Tag color={"green"}>xxx.txt</Tag></TextCircle>
        </div>, value: 2},
    {label: <div>
            <Tag color={"orange"}>输入参数</Tag>此参数在执行命令前需要对此参数进行输入，将输入的值最终转换为固定参数。
            例如：<TextCircle>apt install <Tag color={"orange"}>包名称</Tag></TextCircle>
        </div>, value: 3},
    {label: <div>
            <Tag color={"pink"}>多选参数</Tag> 此参数用于预设一些值，以便在执行名称前提供选择。将最终选择后的值转换为固定参数进行执行。
            例如：
            <TextCircle>
                apt <Tag color={"pink"}>install</Tag>/<Tag color={"pink"}>remove</Tag>/<Tag color={"pink"}>search</Tag> app
            </TextCircle>
        </div>, value: 4},
];


/**
 * 参数输入
 * @constructor
 */
export default function Index() {

    const [args, setArgs] = useState([{ty: ArgType.FIXED, name: '-T'}, {ty: ArgType.FILE, name: 'jar文件'}] as ArgInputValue[]);
    const [cursor, setCursor] = useState(args.length);
    const formValue = useRef<ArgInputValue|null>(null);
    const [openArgModal, setOpenArgModal] = useState(false);
    const autoCompleteRef = useRef<BaseSelectRef>(null);
    const {modal} = AntApp.useApp();
    const [inputValue, setInputValue] = useState('');
    const onEnterInput = (index: number) => {
        let array = [...args];
        let arg: ArgInputValue = {name: inputValue, default_value: inputValue, ty: ArgType.FIXED};
        if (index >= array.length) {
            array.push(arg);
        } else {
            array.splice(index, 0, arg);
        }
        setArgs(array);
        setInputValue('');
        if (cursor >= index) {
            setCursor(Math.max(cursor + 1, array.length));
        }
    };
    // 在光标位置添加参数
    const onAppendOnCursor = (value: ArgInputValue) => {
        let array = [...args];
        let index = cursor;
        if (index >= array.length) {
            array.push(value);
            setCursor(array.length);
        } else {
            array.splice(index, 0, value);
            setCursor(index + 1);
        }
        setArgs(array);
    };
    const onUpdateArg = (arg: ArgInputValue, index: number) => {
        let array = [...args];
        if (array.length > index) {
            array[index] = arg;
            setArgs(array);
        }
    }
    const onArgRemove = (index: number) => {
        let array = [...args];
        array.splice(index, 1);
        if (cursor >= index) {
            setCursor(Math.max(cursor - 1, 0));
        }
        setArgs(array);
    };
    const onAutoCompleteKeyUp = (event: KeyboardEvent<HTMLDivElement>) => {
        if (event.code === 'Enter' && inputValue) {
            onEnterInput(cursor === -1 ? args.length : cursor);
            event.stopPropagation();
            return;
        }
        if (event.code === 'Backspace' && !inputValue && args.length) {
            onArgRemove(cursor === -1 ? args.length - 1 : cursor - 1);
            event.stopPropagation();
            return;
        }
    };
    const onSelect = (ty: string) => {
        const type = parseInt(ty) as ArgType;
        let model = {ty: type, name: '', default_value: ''};
        setInputValue('');
        formValue.current = model;
        setOpenArgModal(true);

    };
    useEffect(() => {
        if (openArgModal) {
            const instance = modal.confirm({
                title: '添加参数',
                icon: null,
                width: '400px',
                content: <ArgConfig value={formValue.current!} onChange={val => {
                    formValue.current = val;
                }}/>,
                onOk: () => {
                    onAppendOnCursor(formValue.current!);
                    instance.destroy();
                    formValue.current = null;
                    setOpenArgModal(false);
                },
                onCancel: () => {
                    instance.destroy();
                    formValue.current = null;
                    setOpenArgModal(false);
                }
            });
        }
    }, [openArgModal]);
    const renderArg = (offset: number) => {
        return (it: ArgInputValue, index: number) => {
            return <Tag key={offset + index}
                        className={`arg-input_item`}
                        onClick={() => {setCursor(offset + index + 1); autoCompleteRef.current?.focus()}}
                        color={ArgTypeColorMap[it.ty]}>{it.name || it.default_value}</Tag>

        }
    };
    return <div className={"arg-input"}>
        光标:{cursor} 参数个数: {args.length}
         <Row>
             {cursor !== 0 && <Col>
                 <div >{args.slice(0, cursor).map(renderArg(0))}</div>
             </Col>}
             <Col flex={1}>
                 <AutoComplete options={ArgInputOptions}
                               ref={autoCompleteRef}
                               onChange={setInputValue}
                               value={inputValue}
                               onKeyDown={onAutoCompleteKeyUp}
                               onSelect={onSelect}
                               placeholder={"请输入参数 空格添加 格式: 参数、参数名称:参数默认值"} size={"small"}/>
             </Col>
             {cursor !== -1 && <Col>
                 <div >{args.slice(cursor).map(renderArg(cursor))}</div>
             </Col>}
         </Row>
        <Row>
            {args.map((it, index) => <Col span={6}>
                <BlockCircle color={ArgTypeColorMap[it.ty]} style={{padding: '4px', marginBottom: '8px'}} key={`${index}`}>
                   <ArgConfig value={it} onChange={model => {
                       onUpdateArg(model, index);
                   }}/>
                </BlockCircle>
            </Col>)}
        </Row>
        <br/>
    </div>;
}
