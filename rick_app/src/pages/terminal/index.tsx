import {App, Card, Tabs} from "antd";
import {Terminal} from 'xterm';
import 'xterm/css/xterm.css';
import {useEffect, useRef, useState} from "react";
import {TerminalModel} from "./terminal-model";
import {createTerminal, killTerminal, listTerminal, readTerminal, writeTerminal} from "./terminal-request";
import {commonProcess, errorMessage} from "../../model";
import {genCode} from "../../utils/uid";
import {listen} from "../../utils/invoke";
import {UnlistenFn} from "@tauri-apps/api/event";
import {stringToArray} from "./terminal-data-utils";


const TerminalMap: Record<string, Terminal> = {};
const TerminalHasListenMap: Record<string, boolean> = {};
let index = 0;
function TerminalPlane(props: { id: string }) {

    const divRef = useRef<HTMLDivElement>(null);
    let terminal = TerminalMap[props.id];
    let id = index ++ ;
    const firstRead = () => {
        if (terminal) {
            commonProcess(readTerminal({id: props.id, offset: 0})).then((data) => {
                for (let i = 0; i < data.length; i++) {
                    let x = data[i];
                    if (x === 127) {
                        data.splice(i, 1, 8, 32, 8);
                        i +=2;
                    }
                }
                terminal.write(new Uint8Array(data));
            });
        }
    };
    useEffect(() => {
        let dispose = null as Promise<UnlistenFn>|null;
        console.log(props.id, TerminalMap, divRef.current, divRef.current && !TerminalMap[props.id] );
        if (divRef.current && !TerminalMap[props.id] ) {
            terminal = new Terminal({rows: 46, cols: 600});
            // @ts-ignore
            window.terminal = terminal;
            TerminalMap[props.id] = terminal;
            terminal.open(divRef.current);
            terminal.onData(async (data) => {
                if (data === '\x7F') {
                    data = '\b';
                }
                await writeTerminal({id: props.id, data: stringToArray(data)});
            });
            if (!TerminalHasListenMap[props.id]) {
                TerminalHasListenMap[id] = true;
                dispose = listen<{ data: number[], id: string }>('terminal:data', data => {
                    console.log(index, "DATA", data.payload);
                    if (data.payload.id === props.id) {
                        let output = data.payload.data;
                        for (let i = 0; i < output.length; i++) {
                            let x = output[i];
                            if (x === 127 || x === 8) {
                                output.splice(i, 1, 8, 32, 8);
                                i +=2;
                            }
                        }
                        const textDecoder = new TextDecoder('gbk');
                        const content = textDecoder.decode(new Uint8Array(output));
                        terminal?.write(content.replace(/(\r)?\n/g, '\r\n'));
                    }
                });
            }
            firstRead();
        }
        return () => {
            terminal?.dispose();
            dispose?.then(it => {
                it();
                delete TerminalHasListenMap[props.id];
            });
            delete TerminalMap[props.id];
        };
    }, []);
    return <div ref={divRef}>

    </div>;
}


export default function TerminalPage() {

    const {message} = App.useApp();
    const [list, setList] = useState([] as TerminalModel[]);

    const load = () => {
        commonProcess(listTerminal()).then(list => {
            setList(list);
            if (!list.length) {
                create();
            }
        }).catch(errorMessage(message));
    };
    const create = () => {
        let id = genCode();
        console.log('id');
        let model = {id: id, name: 'PowerShell'};
        commonProcess(createTerminal(model)).then(model => {
            load();
        }).catch(errorMessage(message));
    };

    const kill = (id: string) => {
        killTerminal({id: id}).then(() => {
            load();
        });
    };

    useEffect(() => {
        setTimeout(load, 100);
    }, []);

    return <div className={"terminal-page"}>
        <Card>
            <Tabs type="editable-card" size={"small"}
                  onEdit={(id, action) => {
                      if (action === 'add') {
                          create();
                      } else {
                          kill(id as string);
                      }
                  }}
                  items={list.map(it => ({
                      key: it.id,
                      label: it.name,
                      children: <TerminalPlane id={it.id}/>
                  }))}/>
        </Card>
    </div>;
}
