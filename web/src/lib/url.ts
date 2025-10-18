/**
 * Get the base URL for assets, respecting PUBLIC_BASE_URL environment variable
 */
export function getBaseUrl(): string {
  // During build, use import.meta.env
  return import.meta.env.PUBLIC_BASE_URL || '';
}

/**
 * Generate full URL for texture/asset paths
 * @param url - Relative URL path (e.g., "assets/minecraft/textures/item/sample.png")
 * @returns Full URL with base path applied
 */
export function assetUrl(url: string): string {
  const base = getBaseUrl();

  // If no base URL, return as-is (local development)
  if (!base || base === '/') {
    return url;
  }

  // Remove leading slash from url if present
  const cleanUrl = url.startsWith('/') ? url.slice(1) : url;

  // Combine base and url, ensuring single slash
  return `${base}/${cleanUrl}`.replace(/\/+/g, '/');
}
