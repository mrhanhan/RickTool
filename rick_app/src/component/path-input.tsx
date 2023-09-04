import {Input, Spin, App} from "antd";
import {ChangeEvent, FormEvent, useEffect, useState} from "react";
import {FileFilled} from "@ant-design/icons";
import {InputProps} from "antd/es/input/Input";
import {commonProcess, errorMessage} from "../model";
import {dirDialogOpen, fileReadDialogOpen} from "../utils/common";

export default function PathInput(props: InputProps & {mode?: 'file'|'dir', title?: string,
    onChange?: (value: string) => void,
    filter?: {name: string, extensions?: string[]}[]}) {
    const [loading, setLoading] = useState(false);
    const {message} = App.useApp();
    const [innerValue, setInnerValue] = useState(props.value);
    useEffect(() => {
        setInnerValue(props.value);
    }, [props.value]);
    const onSelect = () => {
      if (props.mode === 'dir') {
          setLoading(true);
          commonProcess(dirDialogOpen({title: props.title})).then((paths) => {
              setLoading(false);
              if (paths.length) {
                  setInnerValue(paths[0].path);
                  props.onChange?.(paths[0].path);
              }
          }).catch((reason) => {
              setLoading(false);
              errorMessage(message)(reason);
          });
      } else {
          console.log("查询");
          setLoading(true);
          commonProcess(fileReadDialogOpen({title: props.title, filter: props.filter})).then((paths) => {
              setLoading(false);
              if (paths.length) {
                  setInnerValue(paths[0].path);
                  props.onChange?.(paths[0].path);
              }
          }).catch((reason) => {
              setLoading(false);
              errorMessage(message)(reason);
          });
      }
    };
    const inputProps = {
        onInput: (e: FormEvent<HTMLInputElement>) => {
            setInnerValue(e.currentTarget.value);
            props.onChange?.(e.currentTarget.value);
        },
        onChange: (e: ChangeEvent<HTMLInputElement>) => {
            setInnerValue(e.currentTarget.value);
            props.onChange?.(e.currentTarget.value);
        }

    }
    return <Spin spinning={loading}>
        <Input {...{value: innerValue, ...props, ...inputProps}}   addonAfter={<FileFilled onClick={onSelect}/>}/>
    </Spin>
}
