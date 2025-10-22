/**
 * アニメーションデータ
 */
export interface AnimationData {
  /** フレーム数 */
  frame_count: number;
  /** フレームタイム（tick数） */
  frametime: number;
}

/**
 * モデルデータ
 */
export interface ModelData {
  /** モデル名（カスタムモデルデータ名） */
  name: string;
  /** 適用されているマテリアルのリスト */
  materials: string[];
  /** テクスチャパス（3Dモデルの場合は undefined） */
  texture_path?: string;
  /** 追加日時（ISO 8601形式） */
  added_date: string;
  /** アニメーション情報（アニメーションテクスチャの場合のみ） */
  animation?: AnimationData;
}

/**
 * models.json の構造
 */
export interface ModelsJson {
  /** モデルデータの配列 */
  models: ModelData[];
  /** モデルの総数 */
  count: number;
}

/**
 * PRの情報
 */
export interface LatestPr {
  /** PR番号 */
  number: number;
  /** PRタイトル */
  title: string;
  /** PRのURL */
  url: string;
}

/**
 * metadata.json の構造
 */
export interface MetadataJson {
  /** バージョン文字列 */
  version: string;
  /** SHA1ハッシュ */
  sha1: string;
  /** ファイルサイズ（bytes） */
  size: number;
  /** コミットハッシュ */
  commit: string;
  /** 更新日時（ISO 8601形式） */
  updated_at: string;
  /** 最新のPR情報 */
  latest_pr?: LatestPr;
}
