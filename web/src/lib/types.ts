export interface AnimationData {
  frame_count: number;
  frametime: number;
}

export interface ModelData {
  name: string;
  materials: string[];
  texture_url: string;
  added_date: string;
  author: string;
  animation?: AnimationData;
}

export interface ModelsJson {
  models: ModelData[];
}

export interface MetadataJson {
  size: number;
  updated_at: string;
  sha1: string;
  download_url: string;
  latest_pr?: {
    number: number;
    url: string;
  };
}
