import {Button} from "antd";
import {invoke} from "@tauri-apps/api";

export default function AppPage() {
    return <Button onClick={() => invoke('test', {name: 'hello'}).then(resp => console.log(resp))}>Hello</Button>
}
