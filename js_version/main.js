import { ExecutionChain } from "./execution_chain.js"
import { InputStream } from "./input_stream.js";
import { tokenize } from "./tokenizer.js";
import { generateAst } from "./ast.js";

const inputElem = document.querySelector("#input");
const tokensElem = document.querySelector("#tokens");
const astElem = document.querySelector("#ast");

function parse(str) {
    const stream = new InputStream(str);
    // let tokenized;
    // try {
    //     tokenized = tokenize(stream);
    //     tokensElem.value = JSON.stringify(tokenized, null, 2);
    // } catch (err) {
    // }
    new ExecutionChain()
        .then((str) => {
            const stream = new InputStream(str);
            const tokenized = tokenize(stream);
            tokensElem.value = JSON.stringify(tokenized, null, 2);
            return [ new InputStream(tokenized) ];
        }, (err) => {
            tokensElem.value = err.message;
            astElem.value = "";
        })
        .then((tokenizedStream) => {
            const ast = generateAst(tokenizedStream);
            astElem.value = JSON.stringify(ast, null, 2);
            return [ ast ];
        }, (err) => {
            astElem.value = err.message;
        })
        .execute(str);
}

inputElem.addEventListener("input", () => parse(inputElem.value));
parse(inputElem.value);
