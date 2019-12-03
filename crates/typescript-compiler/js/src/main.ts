import ts from "typescript";

export function transpile(source: string, options: ts.TranspileOptions): ts.TranspileOutput {
    return ts.transpileModule(source, options);
}