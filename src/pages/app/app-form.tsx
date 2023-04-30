import { DeleteOutlined, PlusOutlined } from "@ant-design/icons";
import { Alert, Button, Checkbox, Col, Form, Image, Input, Row, Select } from "antd";
import ShortcutKeyInput from "../../components/form/shortcut-key-input";
import { EnvironmentItem, InputType, VApp, VAppArg } from "../../store/store";
import { genId } from "../../utils/id";
import {copy} from "../../utils/copy";


export declare interface AppFormContext {
    /**
     * 提交
     */
    submit(): any;
    /**
     * 取消
     */
    cancel(): any;
}

type AppFormInstanceImpl = AppFormContext & {
    delegate: AppFormContext | null;
}

export declare type VAppFormProps = {

    form?: AppFormContext,
    onSubmit: (app: VApp) => void;
    defaultValue?: VApp;
};

export function createAppFormContext(): AppFormContext {
    const instance: AppFormInstanceImpl = {
        delegate: null,
        submit: () => {
            instance.delegate?.submit();
        },
        cancel: () => {
            instance.delegate?.cancel();
        }
    }
    return instance;
}

function AppIcon(props: { value?: string; onChange?: (value: string) => void }) {
    return <div>
        <Image
            width={64}
            height={64}
            src={props.value === 'error' ? '' : props.value}
            fallback="/default_vapp_icon.png"
        />
    </div>;
}

function VAppArgs(props: { value?: VAppArg[]; onChange?: (value: VAppArg[]) => void }) {
    const { value } = props;
    const onAddArg = () => {
        const arg: VAppArg = {
            id: genId(),
            inputType: 'NORMAL'
        };
        props.onChange?.([...(props.value || []), arg]);
    };
    const onDelArg = (index: number) => {
        const args = [...(props.value || [])]
        args.splice(index, 1);
        props.onChange?.(args);
    };
    const onInputTypeChange = (index: number, value: InputType) => {
        const args = copy<VAppArg[]>(props.value || [])
        args[index].inputType = value;
        props.onChange?.(args);
    };
    const onInputOpt = (index: number, value: string) => {
        const args = copy<VAppArg[]>(props.value || [])
        args[index].opt = value;
        props.onChange?.(args);
    };
    const onInputValue = (index: number, value: string) => {
        const args = copy<VAppArg[]>(props.value || [])
        args[index].value = value;
        props.onChange?.(args);
    };
    const onInputName = (index: number, value: string) => {
        const args = copy<VAppArg[]>(props.value || [])
        args[index].name = value;
        props.onChange?.(args);
    };
    return <div>
        {
            value?.map((it, index) => {
                return <Row key={it.id} style={{ marginBottom: '8px' }}>
                    <Col flex="80px">
                        <Input autoComplete="no" placeholder="参数名称"
                               value={it.name}
                               onInput={(value) => onInputName(index, value.currentTarget.value)}
                        />
                    </Col>
                    <Col>
                        <Select defaultValue={it.inputType} onChange={(value) => onInputTypeChange(index, value)}>
                            <Select.Option label="文件" value="FILE">文件</Select.Option>
                            <Select.Option label="正常" value="NORMAL">正常</Select.Option>
                            <Select.Option label="指令" value="INSTRUCTION">指令</Select.Option>
                        </Select>
                    </Col>
                    <Col flex="1">
                        <Input autoComplete="no" placeholder="参数: -i、--target"
                               value={it.opt}
                               onInput={(value) => onInputOpt(index, value.currentTarget.value)}
                        />
                    </Col>
                    {it.inputType === 'NORMAL' && <Col flex="1">
                        <Input autoComplete="no" placeholder="参数值" value={it.value} onInput={(value) => onInputValue(index, value.currentTarget.value)} />
                    </Col>}
                    <Col>
                        <Button icon={<DeleteOutlined />} onClick={() => onDelArg(index)}></Button>
                    </Col>
                </Row>
            })
        }
        <Button block={true} icon={<PlusOutlined />} onClick={onAddArg}>添加参数</Button>
    </div>
}

function VAppEnv(props: { value?: EnvironmentItem[]; onChange?: (value: EnvironmentItem[]) => void }) {
    const addEnv = () => {
        props.onChange!([...(props.value || []), {key: '', value: ''}]);
    };
    const onDelEnv = (index: number) => {
        const val = copy(props.value!);
        val.splice(index, 1);
        props.onChange!(val);
    };
    const onValueInput = (index: number, e: {currentTarget: { value: string}}) => {
        const val = copy(props.value!);
        val[index].value = e.currentTarget.value;
        props.onChange!(val);
    };
    const onKeyInput = (index: number, e: {currentTarget: { value: string}}) => {
        const val = copy(props.value!);
        val[index].key = e.currentTarget.value;
        props.onChange!(val);
    }
    return <div>
        <Alert type="success" message={<div>
            若环境包含多个值请使用 ';' 分割。在运行时，配置的系统环境优于系统配置环境。若需要使用环境变量：$PATH, $[环境名称]
        </div>}></Alert>
        {
            props.value?.map((it, index) => <div key={`env_${index}`} style={{marginBottom: '8px'}}>
                <Row>
                    <Col flex="100px">
                        <Input placeholder="请输入环境名称, PATH, JAVA_HOME" autoComplete="no" value={it.key} onInput={(e) => onKeyInput(index, e)}/>
                    </Col>
                    <Col flex="auto">
                        <Input placeholder="请输入程序运行环境;分割" autoComplete="no" value={it.value} onInput={(e) => onValueInput(index, e)}/>
                    </Col>
                    <Col>
                        <Button icon={<DeleteOutlined />} onClick={() => onDelEnv(index)}></Button>
                    </Col>
                </Row>
            </div>)
        }
        <Button block={true} icon={<PlusOutlined />} onClick={addEnv}>添加运行环境</Button>
    </div>
}

function VAppForm(props: VAppFormProps) {
    const [form] = Form.useForm();
    if (props.form) {
        (props.form as AppFormInstanceImpl).delegate = {
            submit: () => {
                const app = {...props.defaultValue, ...form.getFieldsValue()}  as VApp;
                console.log(app);
                props.onSubmit(app);
                
            },
            cancel: () => {
                console.log('取消');
            }
        }
    }
    return <Form labelCol={{ span: 4 }} form={form}>
        <Form.Item name="icon"
            label="图标"
            preserve={false}
            initialValue={props.defaultValue?.icon || 'error'}>
            <AppIcon />
        </Form.Item>
        <Form.Item name="name" label="名称"
            preserve={false}
            required={true}
            initialValue={props.defaultValue?.name}>
            <Input autoComplete="no"/>
        </Form.Item>
        <Form.Item label="目标" required={true} style={{ marginBottom: '0px' }}>
            <Row>
                <Col>
                    <Form.Item name="targetType"
                        preserve={false}
                        required={true}
                        initialValue={props.defaultValue?.targetType || 'APP'}>
                        <Select>
                            <Select.Option value="URL" label="URL">JAVA</Select.Option>
                            <Select.Option value="APP" label="可执行程序">可执行程序</Select.Option>
                            <Select.Option value="SHELL" label="Shell">Python</Select.Option>
                        </Select>
                    </Form.Item>
                </Col>
                <Col flex="auto">
                    <Form.Item name="target" initialValue={props.defaultValue?.target}>
                        <Input autoComplete="no" />
                    </Form.Item>
                </Col>
            </Row>
        </Form.Item>
        
        <Form.Item label="目录" name="dir" initialValue={props.defaultValue?.dir || ''}>
            <Input placeholder="请输入程序运行目录"/>
        </Form.Item>
        <Form.Item label="参数" name="args" initialValue={props.defaultValue?.args || []}>
            <VAppArgs />
        </Form.Item>
        <Form.Item label="环境" name="environment" initialValue={props.defaultValue?.environment || []}>
            <VAppEnv />
        </Form.Item>
        <Form.Item label="窗口" name="shell" initialValue={props.defaultValue?.shell || false} valuePropName="checked">
            <Checkbox>运行时打开窗口查看结果</Checkbox>
        </Form.Item>
        <Form.Item label="SUDO" name="sudo" initialValue={props.defaultValue?.sudo || false} valuePropName="checked">
            <Checkbox>管理员模式运行</Checkbox>
        </Form.Item>
        <Form.Item label="快捷键" name="shortcutKey" initialValue={props.defaultValue?.shortcutKey || ''}>
            <ShortcutKeyInput />
        </Form.Item>

    </Form>;
}


export default VAppForm;