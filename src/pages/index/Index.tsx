import {Card, Input} from "antd";
import { useDispatch, useSelector } from "react-redux";
import { selectData, selectText, updateText} from "../../store/configSlice";


function IndexPage() {
    const text = useSelector(selectText);
    const data = useSelector(selectData);
    const dispatch = useDispatch();
    return <Card>
        IndexPage: {text}
        <Input value={text} onInput={({currentTarget: {value}}) => {
            dispatch(updateText(value));
        }}/>
        <pre>
            {JSON.stringify(data, null, 4)}
        </pre>
    </Card>
}

export default IndexPage;