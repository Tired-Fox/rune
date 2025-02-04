import { invoke, type InvokeArgs, type InvokeOptions } from "@tauri-apps/api/core"

export interface TaskOptions {
  signal?: AbortSignal,
}

/// Abortable tauri commands
export async function task(name: string, args?: InvokeArgs, options?: TaskOptions | InvokeOptions): Promise<void> {
  const { signal, ...opts } = options ?? {} as any;
  if (signal?.aborted) {
    throw new Error('task canceled');
  }

  const rid = await invoke<number>(name, args, opts);
  const abort = () => invoke('cancel_task', { rid });

  if (signal?.aborted) {
    abort();
    throw new Error('task canceled');
  }

  signal?.addEventListener("abort", () => void abort());
} 
