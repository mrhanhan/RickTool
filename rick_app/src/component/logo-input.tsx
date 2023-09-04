import {Alert, Image, App as AntApp, Spin, Button} from "antd";
import {useEffect, useState} from "react";
import {fileRead, fileReadDialogOpen} from "../utils/common";
import {commonProcess, errorMessage, Result} from "../model";

export interface LogoInputProps {
    /**
     * 默认Logo路径
     */
    value?: string,
    /**
     * 值
     * @param string
     */
    onChange?: (value: string) =>void
}

export const DEFAULT_LOGO_URL = '/default_vapp_icon.png';

export default function LogoInput(props: LogoInputProps) {
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');
    const [src, setSrc] = useState(DEFAULT_LOGO_URL);
    const {message} = AntApp.useApp();
    const onChangeLogo = () => {
        setLoading(true);
        commonProcess(fileReadDialogOpen({filter: [{name: '*', extensions: ['png', 'jpg', 'jpeg', 'bmp', 'webp']}]})).then((path) => {
            console.log('选择');
            setLoading(false);
            props.onChange?.(path[0].path);
        }).catch(reason => {
            console.log('没选择');
            setLoading(false);
            errorMessage(message)(reason);
        });
    };
    const loadImage = (path: string) => {
        setLoading(true);
        setError('');
        commonProcess(fileRead({path})).then(data => {
            let buffer = new Uint8Array(data).buffer;
            setSrc(URL.createObjectURL(new Blob([buffer])));
        }).catch((reason) => {
            setSrc(DEFAULT_LOGO_URL);
            if (!reason) {
                setError('操作失败');
                return;
            }
            if (typeof reason === 'string') {
                setError(reason);
            } else {
                setError(reason.message);
            }
        }).finally(() =>{
            setLoading(false);
        });
    };
    useEffect(() => {
        return () => {
            if (src !== DEFAULT_LOGO_URL) {
                URL.revokeObjectURL(src);
            }
        }
    }, [src]);
    useEffect(() => {
        if (props.value) {
            loadImage(props.value);
        }
    }, []);
    useEffect(() => {
        if (props.value) {
            loadImage(props.value);
        }
    }, [props.value]);
    return <div style={{textAlign: 'center'}}>
        <Spin spinning={loading}>
            <Image src={src} width={96} height={96}/>
            {error ? <Alert type={"error"} message={error}/> : <br/>}
            <Button onClick={onChangeLogo}>选择</Button>
        </Spin>
    </div>;
}
