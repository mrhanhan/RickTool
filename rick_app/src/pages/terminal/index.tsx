import {App, Card, Tabs} from "antd";
import {Terminal} from 'xterm';
import 'xterm/css/xterm.css';
import {useEffect, useRef, useState} from "react";
import {TerminalModel} from "./terminal-model";
import {createTerminal, killTerminal, listTerminal, readTerminal, writeTerminal} from "./terminal-request";
import {commonProcess, errorMessage} from "../../model";
import {genCode} from "../../utils/uid";
import {listen} from "../../utils/invoke";
import {FitAddon} from 'xterm-addon-fit'
import {UnlistenFn} from "@tauri-apps/api/event";
import {stringToArray} from "./terminal-data-utils";


const TerminalMap: Record<string, Terminal> = {};
const TerminalHasListenMap: Record<string, boolean> = {};
let index = 0;
function TerminalPlane(props: { id: string }) {

    const divRef = useRef<HTMLDivElement>(null);
    let terminal = TerminalMap[props.id];
    let id = index ++ ;
    const [size, setSize] = useState({rows: 40, cols: 100});
    const firstRead = () => {
        if (terminal) {
            commonProcess(readTerminal({id: props.id, offset: 0})).then((data) => {
                terminal.write(new Uint8Array(data));
            });
        }
    };
    useEffect(() => {
        let dispose = null as Promise<UnlistenFn>|null;
        console.log(props.id, TerminalMap, divRef.current, divRef.current && !TerminalMap[props.id] );
        if (divRef.current && !TerminalMap[props.id] ) {
            terminal = new Terminal();
            // @ts-ignore
            window.terminal = terminal;
            const fitAddon = new FitAddon();
            terminal.loadAddon(fitAddon);
            fitAddon.fit();
            TerminalMap[props.id] = terminal;
            terminal.open(divRef.current);
            (terminal as any).fit = fitAddon;
            terminal.onResize((size) => {
                setSize(size);
                console.log(size);
            });
            terminal.onData(async (data) => {
                await writeTerminal({id: props.id, data: stringToArray(data)});
            });
            if (!TerminalHasListenMap[props.id]) {
                TerminalHasListenMap[id] = true;
                dispose = listen<{ data: number[], id: string }>('terminal:data', data => {
                    if (data.payload.id === props.id) {
                        let output = data.payload.data;
                        const textDecoder = new TextDecoder('utf-8');
                        const content = textDecoder.decode(new Uint8Array(output));
                        terminal?.write(content);
                    }
                });
            }
            firstRead();
        }
        return () => {
            terminal?.dispose();
            (terminal as any)?.fit?.dispose()
            dispose?.then(it => {
                it();
                delete TerminalHasListenMap[props.id];
            });
            delete TerminalMap[props.id];
        };
    }, []);
    return <div>
        <div ref={divRef}></div>
        <div>ROWS:{size.rows} COLS:{size.cols}</div>
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
