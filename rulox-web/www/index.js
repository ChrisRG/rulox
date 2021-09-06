import * as wasm from "../pkg";

import { CodeJar } from 'codejar';
import { withLineNumbers } from 'codejar/linenumbers';
import { gruvboxesque } from "./code-editor.js";

const codeEditor = document.querySelector('#code-editor');
const codeJar = CodeJar(codeEditor, withLineNumbers(gruvboxesque));


const compileBtn = document.getElementById("compile-btn");
const tokenArea = document.getElementById("token-output-area");
const astArea = document.getElementById("ast-output-area");
const envArea = document.getElementById("env-output-area");
const resultArea = document.getElementById("result-output-area");

const callCompiler = () => {
    const code = codeJar.toString();
    console.log("Running compiler...");
    return wasm.run(code);
}

// Draw underline beneath lines with errors
const processErrorLines = ((errorIndexes) => {
    let newCode = codeJar.toString().split("\n");
    errorIndexes.forEach((err) => {
        let [line, _] = err;
        // Need to rework underlining certain sections
        // newCode[line] = newCode[line].substring(0, col - 1) + '<u>' + newCode[line].substring(col - 1) + '</u>';
        newCode[line] = '<u>' + newCode[line] + '</u>';
    });
    codeJar.updateCode(newCode.join("\n"));
});

// Retrieve error information => array of array[line of error, column of error]
const getErrorIndexes = ((errorMsgs) => {
    let indexes = [];
    errorMsgs.forEach((elem) => {
        const matchLine = elem.match(/\bLine (\d+)/);
        const matchCol = elem.match(/\bCol (\d+)/);
        if (matchLine !== null && matchCol !== null) {
            const errLine = Number(matchLine[1] - 1);
            const errCol = Number(matchCol[1]);
            indexes.push([errLine, errCol]);
        }
    });
    return indexes;
});

compileBtn.addEventListener('click', () => {
    const rulox = callCompiler();
    const tokens = rulox.tokens();
    const parseTree = rulox.parse_tree();
    const output = rulox.interpret();
    const env = rulox.get_environment();
    if (rulox.had_errors()) {
        resultArea.innerHTML = "<pre>Compilation error.\n" + output.join("\n") + "</pre>";
        const errIndexes = getErrorIndexes(output);
        processErrorLines(errIndexes);
    } else {
        tokenArea.innerHTML = "<pre>" + JSON.stringify(tokens, null, 2) + "</pre>";
        astArea.innerHTML = "<pre>" + JSON.stringify(parseTree, null, 2) + "</pre>";
        resultArea.innerHTML = "<pre>" + output.join("\n") + "</pre>";
        envArea.innerHTML = "<pre>" + env + "</pre>";
    }
});

