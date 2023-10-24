import {Card} from "antd";
import {MemoryUnit} from "../../assembly_simulation/base";

// @ts-ignore
window.memorgUnit = new MemoryUnit(16);
export default function HomePage() {

    return <div className={"home-page"}>
        <Card>
            <div className={"terminal"}>

            </div>
        </Card>
    </div>
}
