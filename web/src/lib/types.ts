export interface AnimationData {
  frame_count: number;
  frametime: number;
}

export interface ModelData {
  name: string;
  materials: string[];
  // undefined だったら、3Dモデルのため、表示しない
  texture_path?: string;
  added_date: string;
  animation?: AnimationData;
}

export interface ModelsJson {
  models: ModelData[];
}

export interface MetadataJson {
  size: number;
  updated_at: string;
  sha1: string;
  latest_pr?: {
    number: number;
    url: string;
  };
}
