const { core } = Deno;
const { ops } = core;

function argsToMessage(...args) {
  return args.map((arg) => JSON.stringify(arg)).join(" ");
}

const console = {
  log: (...args) => {
    core.print(`[MESSAGE]: ${argsToMessage(...args)}\n`, false);
  },
  error: (...args) => {
    core.print(`[ERROR]: ${argsToMessage(...args)}\n`, true);
  },
};

const girlfriend = {
  readFile: (path) => {
    // return ops.op_read_file(path);
    return core.opAsync("op_read_file", path);
  },
  writeFile: (path, contents) => {
    return ops.op_write_file(path, contents);
  },
  removeFile: (path) => {
    return ops.op_remove(path);
  },
  fetch: async (url) => {
    return core.opAsync("op_fetch", url);
  },
  mkdir: (path) => {
    return ops.op_make_directory(path);
  },
  cd: (path) => {
    return ops.op_current_directory(path);
  },
  ls: (path) => {
    return ops.op_list_directory(path);
  },
};

globalThis.console = console;
globalThis.girlfriend = girlfriend;
globalThis.cat = girlfriend.readFile;
globalThis.rm = girlfriend.removeFile;
globalThis.wget = girlfriend.fetch;
globalThis.echo = girlfriend.writeFile;
globalThis.mkdir = girlfriend.mkdir;
globalThis.cd = girlfriend.cd;
globalThis.ls = girlfriend.ls;