import { PlusCircleOutlined, ReloadOutlined } from "@ant-design/icons";
import {Card, List, Button} from "antd";
import { useState, useEffect } from "react";
import { EnvironmentGroup } from "../../service/environment";
import { getEnvs } from "../../service/EnvironmentService";


function EnvironmentPage() {
    const [envs, setEnvs] = useState([] as EnvironmentGroup[]);
    const [loading, setLoading] = useState(true);
    const onLoad = () => {
        setLoading(true);
        getEnvs().then(envs => {
            setEnvs(envs);
        }).finally(() => {
            setLoading(false);
        });
    };
    useEffect(() => {
       onLoad();
    }, []);
    const extra = <Button.Group >
        <Button icon={<ReloadOutlined />} loading={loading}  onClick={() => onLoad()}/>
        <Button icon={<PlusCircleOutlined/>}/>
    </Button.Group>;
    return <Card title={"环境变量"} extra={extra} loading={loading} >
        <List dataSource={envs} renderItem={({name}) => <List.Item>
            {name}
        </List.Item>} />
    </Card>
}

export default EnvironmentPage;