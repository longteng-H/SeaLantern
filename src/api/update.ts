import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

export interface UpdateInfo {
  has_update: boolean;
  latest_version: string;
  current_version: string;
  download_url?: string;
  release_notes?: string;
  published_at?: string;
}

export interface UpdateProgress {
  downloaded: number;
  total: number;
}

/**
 * 检查更新（使用 Tauri Updater）
 */
export async function checkUpdate(): Promise<UpdateInfo | null> {
  try {
    const update = await check();

    if (update) {
      return {
        has_update: true,
        latest_version: update.version,
        current_version: update.currentVersion,
        release_notes: update.body,
        published_at: update.date,
      };
    }

    return null;
  } catch (error) {
    console.error('检查更新失败:', error);
    throw error;
  }
}

/**
 * 下载并安装更新
 * @param onProgress 下载进度回调
 */
export async function downloadAndInstall(
  onProgress?: (progress: UpdateProgress) => void
): Promise<void> {
  try {
    const update = await check();

    if (!update) {
      throw new Error('没有可用的更新');
    }

    // 下载并安装
    await update.downloadAndInstall((event) => {
      switch (event.event) {
        case 'Started':
          console.log('开始下载更新...');
          if (onProgress) {
            onProgress({ downloaded: 0, total: event.data.contentLength || 0 });
          }
          break;
        case 'Progress':
          console.log(`下载进度: ${event.data.chunkLength} bytes`);
          if (onProgress && event.data.chunkLength) {
            onProgress({
              downloaded: event.data.chunkLength,
              total: event.data.contentLength || 0
            });
          }
          break;
        case 'Finished':
          console.log('下载完成');
          break;
      }
    });

    console.log('更新已安装，准备重启...');
  } catch (error) {
    console.error('下载安装失败:', error);
    throw error;
  }
}

/**
 * 重启应用
 */
export async function restartApp(): Promise<void> {
  await relaunch();
}
