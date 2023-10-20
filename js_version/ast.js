export function generateAst(stream) {
    const res = [];
    let item = readNext(stream);
    while (item !== null) {
        res.push(item);
        item = readNext(stream);
    }
    return res;
}

function readNext(stream) {
    if (stream.eof(false)) return null;
    const token = stream.peek();
    if (token.type === "token" && token.specific === "divide") return readCommand(stream);
    stream.yeet(`Unexpected token '${JSON.stringify(token)}'`);
}

function readCommand(stream) {
    const position = stream.next().position;
    const res = stream.until((token) => token.type === "token" && token.specific === "end");
    return { "type": "command", "value": res };
}
