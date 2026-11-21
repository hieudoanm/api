import * as fs from 'node:fs';
import * as path from 'node:path';
import * as vscode from 'vscode';
import { convert, convert_all, formats, initSync } from '@api/wasm';

const SPEC_RE = /\.(yaml|yml|json)$/i;
const wasmPath = path.join(__dirname, 'api_wasm_bg.wasm');

let ready = false;
const ensureWasm = (): void => {
  if (!ready) {
    initSync(fs.readFileSync(wasmPath));
    ready = true;
  }
};

interface ConversionFile {
  format: string;
  path: string;
  content: string;
}

interface ConversionResult {
  files: ConversionFile[];
}

const parseFiles = (json: string): ConversionFile[] =>
  (JSON.parse(json) as ConversionResult).files;

const pickSpec = async (): Promise<vscode.Uri | undefined> => {
  const editor = vscode.window.activeTextEditor;
  if (
    editor?.document.uri.scheme === 'file' &&
    SPEC_RE.test(editor.document.uri.fsPath)
  ) {
    return editor.document.uri;
  }
  const picked = await vscode.window.showOpenDialog({
    canSelectMany: false,
    canSelectFolders: false,
    filters: { 'OpenAPI specs': ['yaml', 'yml', 'json'] },
    title: 'Select an OpenAPI spec to convert',
  });
  return picked?.[0];
};

const pickFormat = async (): Promise<string | undefined> => {
  const all = (JSON.parse(formats()) as string[]).map((f) => ({
    label: f,
    value: f,
  }));
  all.push({ label: 'all (every format)', value: 'all' });
  const picked = await vscode.window.showQuickPick(all, {
    placeHolder: 'Select the target API client format',
  });
  return picked?.value;
};

const writeFiles = async (
  specDir: string,
  files: ConversionFile[]
): Promise<number> => {
  let count = 0;
  for (const file of files) {
    const target = path.join(
      specDir,
      'api-collections',
      file.format,
      file.path
    );
    await vscode.workspace.fs.createDirectory(
      vscode.Uri.file(path.dirname(target))
    );
    await vscode.workspace.fs.writeFile(
      vscode.Uri.file(target),
      new TextEncoder().encode(file.content)
    );
    count += 1;
  }
  return count;
};

const convertSpec = async (spec: vscode.Uri): Promise<void> => {
  const source = new TextDecoder().decode(
    await vscode.workspace.fs.readFile(spec)
  );
  ensureWasm();

  const format = await pickFormat();
  if (!format) return;

  const json =
    format === 'all'
      ? convert_all(source, '', '')
      : convert(source, format, '', '');
  const files = parseFiles(json);
  const count = await writeFiles(path.dirname(spec.fsPath), files);

  vscode.window.showInformationMessage(
    `api: wrote ${count} file(s) to api-collections/ next to ${path.basename(spec.fsPath)}`
  );
};

export function activate(context: vscode.ExtensionContext): void {
  context.subscriptions.push(
    vscode.commands.registerCommand(
      'api.convertSpec',
      async (arg?: vscode.Uri) => {
        try {
          // Menu invocations pass the clicked resource; the palette passes
          // nothing and falls back to the active editor / open dialog.
          const spec = arg instanceof vscode.Uri ? arg : await pickSpec();
          if (!spec) return;
          await convertSpec(spec);
        } catch (err) {
          vscode.window.showErrorMessage(
            `api: conversion failed — ${err instanceof Error ? err.message : String(err)}`
          );
        }
      }
    )
  );
}

export function deactivate(): void {}
