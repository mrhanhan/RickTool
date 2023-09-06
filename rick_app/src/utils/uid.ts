

const IData = {
    count: 0
};

export function getId() {
    let number = Math.random();
    let date = new Date();
    let time = date.getTime();
    time = time * number;
    time = parseInt((time / 1000).toFixed(0)) * 1000;
    time = time + IData.count;
    IData.count ++;
    if (IData.count >= 1000) {
        IData.count = 0;
    }
    return time;
}


export function genCode() {
    return getId().toString(16) + getId().toString(16);
}

export function getNow() {
    return parseInt((new Date().getTime() / 1000).toFixed(0));
}
