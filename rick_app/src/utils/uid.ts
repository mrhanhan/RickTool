

const IData = {
    count: 0
};

export function getId() {
    let number = Math.random() * 100;
    let date = new Date();
    let time = date.getTime();
    time /= 1000;
    time *= 100;
    time = time + IData.count;
    IData.count ++;
    if (IData.count >= 1000) {
        IData.count = 0;
    }
    return parseInt(time.toFixed(0));
}


export function getNow() {
    return parseInt((new Date().getTime() / 1000).toFixed(0));
}
