export function tokenize(stream) {
    const res = [];
    let token = readNext(stream);
    while (token !== null) {
        res.push(token);
        token = readNext(stream);
    }
    return res;
}

const tokens = {
    "+": "add",
    "-": "subtract",
    "*": "multiply",
    "/": "divide",
    "%": "modulo",
    "<": "less_than",
    ">": "greater_than",
    "=": "assignment",
    "!": "not",
    "{": "open_scope",
    "}": "close_scope",
    "(": "open_group",
    ")": "close_group",
    "[": "open_index",
    "]": "close_index",
    ".": "property",
    "?": "ternary",
    ":": "ternary_split",
    "==": "equals",
    "<=": "less_than_equals",
    ">=": "greater_than_equals",
    "!=": "not_equals",
    "&&": "and",
    "||": "or",
    "??": "nullish_coalescing",
    ";": "end",
    ",": "separator",
    "->": "lambda"
};
const maxTokenLength = Math.max(
    ...(Object.keys(tokens)
        .map(val => val.length))
);

const keywords = [
    "for",
    "while",
    "let",
    "const",
    "function",
    "as",
    "null",
    "return",
    "throw"
];

function readNext(stream) {
    skipWhitespace(stream);
    if (stream.eof(false)) return null;
    const char = stream.peek();
    if (char === "\"") return readString(stream);
    if (/[a-zA-Z]/.test(char)) return readWordLike(stream);
    {
        const token = readToken(stream);
        if (token !== null) return token;
    }
    if (/[0-9\-]/.test(char)) return readNumber(stream);
    stream.yeet(`Unexpected token '${char}'`);
}

function skipWhitespace(stream) {
    while (true) {
        const next = stream.peek(false);
        if (next === null) return;
        if ([" ", "\t", "\n"].includes(next)) {
            stream.skip();
            continue;
        }
        return;
    }
}

const escapedVersion = {
    "t": "\t",
    "n": "\n"
};
function readString(stream) {
    const position = { row: stream.row, col: stream.col, position: stream.position, tokenLength: NaN };
    stream.skip();
    let escaped = false;
    let string = "";
    while (true) {
        if (stream.eof(false)) stream.yeet("Unexpected end of string literal");
        const char = stream.next();
        if (escaped) {
            string += escapedVersion[char] || char;
            escaped = false;
            continue;
        }
        if (char === "\\") {
            escaped = true;
            continue;
        }
        if (char === "\"") {
            position.tokenLength = string.length + 2;
            return { "type": "string", "specific": "literal", "value": string, "position": position };
        }
        string += char;
    }
}

const floatTypes = {
    "d": "double",
    "f": "float",
};
const intTypes = {
    "i": "int",
    "s": "short",
    "l": "long"
};

function readNumber(stream) {
    const position = { row: stream.row, col: stream.col, position: stream.position, tokenLength: NaN };
    let number = "";
    let hasDecimal = false;
    if (stream.peek() === "-") {
        number += "-";
        stream.skip();
    }
    while (true) {
        if (stream.eof(false)) {
            position.tokenLength = number.length;
            if (hasDecimal) return { "type": "number", "specific": "signed_float", "value": Number(number) || 0, "position": position };
            else return { "type": "number", "specific": "signed_int", "value": Number(number) || 0, "position": position };
        }
        let char = stream.peek();
        if (char === ".") {
            if (hasDecimal) {
                stream.yeet("Unexpected double decimal point");
            }
            hasDecimal = true;
            number += ".";
            stream.skip();
            continue;
        }
        if (/[0-9]/.test(char)) {
            stream.skip();
            number += char;
        } else {
            let specific = "signed_";
            let parsedNumber = Number(number) || 0;
            char = char.toLowerCase();
            if (char === "u") {
                if (number.startsWith("-")) stream.yeet("Unsigned number cannot be negative");
                position.tokenLength = number.length + 2;
                stream.skip();
                parsedNumber = Math.trunc(parsedNumber);
                if (stream.eof(false)) stream.yeet("Ah yes `unsigned EOF` my favorite data type");
                specific = "unsigned_";
                specific += intTypes[stream.peek()] || stream.yeet(`Unknown integer type '${stream.peek()}'`);
                stream.skip();
            } else if (/\s/.test(char) || Object.keys(tokens).includes(char)) {
                position.tokenLength = number.length;
                if (hasDecimal) return { "type": "number", "specific": "signed_float", "value": parsedNumber, "position": position };
                else return { "type": "number", "specific": "signed_int", "value": parsedNumber, "position": position };
            } else {
                position.tokenLength = number.length + 1;
                stream.skip();
                if (intTypes[char]) parsedNumber = Math.trunc(parsedNumber);
                specific += floatTypes[char] || intTypes[char] || stream.yeet(`Unknown number type '${char}'`);
            }
            return { "type": "number", "specific": specific, "value": parsedNumber, "position": position };
        }
    }
}

function readWordLike(stream) {
    const position = { row: stream.row, col: stream.col, position: stream.position, tokenLength: NaN };
    let word = "";
    while (!stream.eof(false) && /[a-zA-Z]/.test(stream.peek())) {
        const char = stream.next();
        word += char;
    }
    position.tokenLength = word.length;
    let specific = "generic";
    return { "type": "word", "specific": specific, "value": word, "position": position };
}

function readToken(stream) {
    if (stream.eof(false)) return null;
    const position = { row: stream.row, col: stream.col, position: stream.position, tokenLength: NaN };
    let token = "";
    lengthLoop: for (let i = maxTokenLength; i > 0; i--) {
        let test = "";
        for (let j = 0; j < i; j++) {
            const val = stream.peek(false, j);
            if (val === null) continue lengthLoop;
            test += val;
        }
        if (Object.keys(tokens).includes(test)) {
            token = test;
            break;
        }
    }
    if (token === "") return null;
    position.tokenLength = token.length;
    for (let i = 0; i < token.length; i++) stream.skip();
    return { "type": "token", "specific": tokens[token], "value": token, "position": position };
}
