import ts from "typescript";

export function transpile(source: string, options: ts.TranspileOptions): ts.TranspileOutput {
    return ts.transpileModule(source, options);
}

export function parse(source: string): ts.SourceFile {
    let sourceFile = ts.createSourceFile("main.ts", source, ts.ScriptTarget.Latest, true);
    return sourceFile;
}