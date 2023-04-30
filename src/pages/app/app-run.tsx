import {Col, Input, Modal, Row, Tag} from "antd";
import Iconfont from "../../components/iconfont";
import {createWindow} from "../../service/invoke/Window";
import {VApp, VAppArg} from "../../store/store";
import {copy} from "../../utils/copy";
import {invokeRunApp} from "../../service/invoke/VApp";

/**
 * 处理APP 运行时参数
 * @param app 需要运行的app
 */
export function processAppRuntimeArgs(app: VApp): Promise<VApp> {
    let inputArgs: VAppArg[] = copy(app.args)!;
    const onInputArg = (index: number, value: string) => {
        inputArgs[index].value = value;
    }
    const onInputArgOpt = (index: number, value: string) => {
        inputArgs[index].opt = value;
    }
    if (!inputArgs.length) {
        return Promise.resolve(app);
    }
    return new Promise<VApp>((resolve) => {
        let newApp = copy(app);
        Modal.confirm({
            icon: <Iconfont icon="yunhang"></Iconfont>,
            title: '请输入运行参数',
            okText: '运行',
            cancelText: '取消',
            content: <div>
                {
                    inputArgs.map((item, index) => <Row style={{marginBottom: '8px'}}>
                        <Col flex="100px"><Tag color={"primary"}>{item.name}</Tag></Col>
                        <Col flex="1 1 0">
                            <Input placeholder={`请输入${item.name}值`}
                                   defaultValue={item.opt}
                                   onInput={(e) => onInputArgOpt(index, e.currentTarget.value)}/>
                        </Col>
                        {item.inputType === 'NORMAL' && <Col flex="1 1 0">
                            <Input placeholder={`请输入${item.name || item.opt}值`}
                                   defaultValue={item.value}
                                   onInput={(e) => onInputArg(index, e.currentTarget.value)}/>
                        </Col>}
                    </Row>)
                }
            </div>,
            onOk: () => {
                newApp.args = inputArgs;
                console.log('参数:', inputArgs);
                resolve(newApp);
            },
            onCancel: () => {
            },
        });
    });
}


export function runApp(app: VApp) {
    // 处理参数 处理有一些需要输入的参数
    processAppRuntimeArgs(app).then((newApp) => {
        // 如果需要，弹出运行结果对话框
        if (app.shell) {
            console.log('创建窗口');
            createWindow(`${app.name}`, "vapp/index.html", 1024, 600).then((id) => {
                // 提交到后台开始运行
                console.log('启动程序', id);
                setTimeout(() => {
                    invokeRunApp(newApp, id).then(() => {
                    });
                }, 1000);
            });
        } else {
            // 提交到后台开始运行
            invokeRunApp(newApp, '').then(() => {
            });
        }
    });
}