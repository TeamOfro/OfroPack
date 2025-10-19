import { base } from '$app/paths';

const baseUrl = base ? base.endsWith('/') ? base : `${base}/` : '/';

export function assetUrl(path: string): string {
  if (path.startsWith('/')) {
    path = path.slice(1);
  }
  if (baseUrl === '' || baseUrl === '/') {
    return path;
  }
  return baseUrl + path;
}
