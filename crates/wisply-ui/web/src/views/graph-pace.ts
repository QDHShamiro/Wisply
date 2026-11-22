export function frame(f: () => void): () => void {
  let queued = false;
  return () => {
    if (queued) return;
    queued = true;
    requestAnimationFrame(() => {
      queued = false;
      f();
    });
  };
}

export function idle(run: () => void, ms: number): () => void {
  if (typeof requestIdleCallback === "function") {
    const id = requestIdleCallback(run, { timeout: ms });
    return () => cancelIdleCallback(id);
  }
  const id = setTimeout(run, ms);
  return () => clearTimeout(id);
}
