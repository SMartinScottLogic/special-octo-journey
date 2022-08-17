const invoke = window.__TAURI__.invoke

export async function invokeHello(root) {
    return await invoke("hello", {root });
}

