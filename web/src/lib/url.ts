export function assetUrl(path: string): string {
  let base = import.meta.env.BASE_URL || '/';
  if (!base.endsWith('/')) {
    base += '/';
  }
  if (path.startsWith('/')) {
    path = path.slice(1);
  }
  if (base === '' || base === '/') {
    return path;
  }
  return base + path;
}
