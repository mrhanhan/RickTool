import { Input } from "antd";



function ShortcutKeyInput(props: { value?: string, onChange?: (value: string) => void }) {
    let keys = (props.value || '').split('+');
    const map: Record<string, boolean> = {};
    keys.forEach(it => map[it] = true);
    const getKey = (key: string) => map[key] ? key : '';
    const isControlKey = (key: string) => {
        return (key === 'Control' || key === 'Shift' || key === 'Alt');
    }
    const lastKey = () => {
        return keys.length > 0 ? keys[keys.length - 1] : '';
    }
    keys = [getKey('Control'), getKey('Shift'), getKey('Alt'), isControlKey(lastKey()) ? '' : lastKey()];
    const onKeyDown = (event: { key: string, keyCode: number, preventDefault: () => void }) => {
        event.preventDefault();
        if (event.key === 'Shift') {
            keys[1] = 'Shift';
        } else if (event.key === 'Alt') {
            keys[2] = 'Alt';
        } else if (event.key === 'Control') {
            keys[0] = 'Control';
        } else {
            if (event.keyCode >= 65 && event.keyCode <= 91) {
                keys[3] = event.key.toLocaleUpperCase();
            } else {
                console.log(event.keyCode);
                keys[3] = '';
            }
        }
        props.onChange!(keys.join('+').replace(/\+\+\+?/g, '+').replace(/^\+/, ''));
    };
    const onClear = (event: { currentTarget: { value: string } }) => {
        if (event.currentTarget.value === '') {
            props.onChange!('');
        }
    };

    return <div>
        <Input onKeyDown={(e) => onKeyDown(e)} value={props.value} placeholder="请输入快捷键" allowClear={true} onChange={onClear} />
    </div>
}

export default ShortcutKeyInput;