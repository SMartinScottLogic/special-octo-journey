const invoke = window.__TAURI__.invoke

export async function invokeReadDir(root) {
    return await invoke("read_dir", {root });
}

export async function invokeReadFile(filename) {
    return await invoke("read_file", {filename });
}

