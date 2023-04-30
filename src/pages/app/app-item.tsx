import { Image } from "antd";
import { VApp, VAppGroup } from "../../store/store";
import './app-item.css';
declare interface AppItemProps {
    app: VApp;
    group: VAppGroup;
    contextMenu?: any
}


function AppItem(props: AppItemProps) {
    const {app} = props;

    // console.log(props.contextMenu);
    return <div className="app-item_wrapper" {...props.contextMenu}>
        <Image src={app.icon === 'error' ? '/default_vapp_icon.png' : app.icon} preview={false} />
        <div className="app-item_app-name">
            {app.name}
        </div>
    </div>;
}

export default AppItem;