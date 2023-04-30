import { ClearOutlined, DownCircleOutlined, EditOutlined, MinusOutlined, PlusCircleOutlined, SaveOutlined, UpCircleOutlined } from "@ant-design/icons";
import { Form, Input, Checkbox, Row, Table, Button, FormInstance } from "antd";
import { EnvironmentGroup, EnvironmentItem } from "../../service/environment";
import { genId } from "../../utils/id";

declare type EnvironmentItemPropsValue = EnvironmentItem & { edit?: boolean };

declare type EnvironmentItemProps = {
    value?: EnvironmentItemPropsValue[],
    onChange?: (value?: EnvironmentItemPropsValue[]) => void
};

function EnvironmentItemInput(props: EnvironmentItemProps) {

    const findReplace = (index: number, change: (value: EnvironmentItemPropsValue) => EnvironmentItemPropsValue) => {
        if (index >= 0 && index < props.value!.length) {
            const value: EnvironmentItemPropsValue[] = [...props.value!];
            let oldValue = value[index];
            value[index] = change(oldValue);
            props.onChange?.(value);
        }
    }

    const onEdit = (index: number) => {
        findReplace(index, (value) => ({ ...value, edit: true }));
    }
    const onSave = (index: number) => {
        findReplace(index, (value) => { delete value.edit; return value; });
    }
    const onInput = (index: number, field: keyof EnvironmentItemPropsValue, v: never) => {
        findReplace(index, (value: EnvironmentItemPropsValue) => { value[field] = v; return value; });
    }
    const onAdd = () => {
        const value: EnvironmentItemPropsValue[] = [...props.value!];
        value.push({
            id: genId(),
            key: '',
            value: '',
            edit: true
        });
        props.onChange?.(value);
    };
    const onClear = () => {
        props.onChange?.([]);
    };
    // 交换位置
    const onSwap = (index: number) => {
        const value: EnvironmentItemPropsValue[] = [...props.value!];
        let temp: EnvironmentItemPropsValue = value[index - 1];
        value[index - 1] = value[index];
        value[index] = temp;
        props.onChange?.(value);
    };
    const flooter = () => <><Button icon={<PlusCircleOutlined />} onClick={() => onAdd()}>添加环境</Button>
        <Button icon={<ClearOutlined />} onClick={() => onClear()}>清空环境</Button></>;

    const renderRow = (field: keyof EnvironmentItemPropsValue, placeholder: string) => {
        return (value: string, record: EnvironmentItemPropsValue, index: number) => 
        (!record.edit ? <span>{value}</span> : 
                        <Input placeholder={placeholder} defaultValue={value} 
                        onChange={({ currentTarget: { value } }) => onInput(index, field, value as never)} />)
    };


    return <Table size="small" footer={flooter} bordered pagination={false}
        rowKey='id' dataSource={props.value}>
        <Table.Column title="操作" width={'100px'} render={(value, record: EnvironmentItemPropsValue, index: number) => {
            return <Button.Group size="small">
                {!record.edit ? <Button icon={<EditOutlined />} onClick={() => { onEdit(index) }} /> : <Button icon={<SaveOutlined />} onClick={() => onSave(index)} />}
                <Button icon={<MinusOutlined />} />
                <Button icon={<UpCircleOutlined />} onClick={() => onSwap(index)} disabled={!index}/>
                <Button icon={<DownCircleOutlined />}  onClick={() => onSwap(index + 1)}  disabled={index === (props.value?.length || 0) - 1}/>
            </Button.Group>
        }}></Table.Column>
        <Table.Column title="环境名称" render={renderRow('key', '请输入环境变量名称')} width={'150px'} dataIndex="key"></Table.Column>
        <Table.Column title="环境值" render={renderRow('value', '请输入环境变量值')} dataIndex="value"></Table.Column>
    </Table>;
}



export default function EnvironmentForm(props: {form: FormInstance<any>, model: EnvironmentGroup}) {
    return <Form layout="vertical" initialValues={props.model}
     form={props.form}>
        <Form.Item label="环境名称:" required={true} name="name">
            <Input placeholder="环境变量名称" />
        </Form.Item>
        <Form.Item label="系统变量:" name="flags">
            <Checkbox.Group>
                <Checkbox value={"1"}>包含系统变量</Checkbox>
                <Checkbox value={"2"}>重复合并</Checkbox>
            </Checkbox.Group>
        </Form.Item>
        <Form.Item label="环境变量" name="items">
            <EnvironmentItemInput />
        </Form.Item>
        <Form.Item label="环境说明:" name="description">
            <Input.TextArea placeholder='当前环境变量的说明' />
        </Form.Item>

    </Form>;
}