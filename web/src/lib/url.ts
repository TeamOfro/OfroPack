const base = import.meta.env.BASE_URL || '/';

export function assetUrl(path: string): string {
  if (!path.startsWith('/')) {
    path = `/${path}`;
  }
  if (base === '' || base === '/') {
    return path;
  }
  return base + path;
}
