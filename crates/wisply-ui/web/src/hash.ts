function plain(part: string): string {
  try {
    return decodeURIComponent(part);
  } catch {
    return part;
  }
}

export function hashParts(hash: string): string[] {
  return hash.slice(2).split("?")[0].split("/").map(plain);
}
