const invoke = window.__TAURI__.invoke

export async function invokeReadDir(root) {
    return await invoke("read_dir", {root });
}

