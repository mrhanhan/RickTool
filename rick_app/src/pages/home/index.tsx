import {Card} from "antd";
import {useEffect, useRef} from "react";
import {Terminal} from "xterm";
import {FitAddon} from 'xterm-addon-fit'

export default function HomePage() {

    const terminalDivRef = useRef<HTMLDivElement | null>(null);

    useEffect(() => {
        let terminal = new Terminal({

        });
        const fitAddon= new FitAddon();
        terminal.loadAddon(fitAddon);
        terminal.open(terminalDivRef.current!);
        terminal.onData((data) => {
            console.log('Data', data, data.length);
            terminal.write(data);
            // terminal.write("\r\n");
        });
        terminal.onKey((data) => {
            console.log('Key', data);
            if (data.key === '\x7F') {
                terminal.write("\b \b");
            }
        })
        fitAddon.fit();
        return () => {
            terminal.dispose();
            fitAddon.dispose();
        }
    }, []);

    return <div className={"home-page"}>
        <Card>
            <div className={"terminal"} ref={terminalDivRef}>

            </div>
        </Card>
    </div>
}
