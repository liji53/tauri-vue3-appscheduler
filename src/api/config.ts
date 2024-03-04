import {
  readTextFile,
  writeTextFile,
  exists,
  createDir
} from "@tauri-apps/api/fs";
import { join, localDataDir } from "@tauri-apps/api/path";

// type configResult = {
//   apps_url: string;
//   app_user: string;
//   app_passwd: string;
// };

/** 应用配置 */
export const getConfig = async () => {
  const localDataPath = await localDataDir();
  const path = await join(localDataPath, "appscheduler");
  const config_file = await join(path, "config.json");
  const contents = await readTextFile(config_file);
  return JSON.parse(contents);
};

export const setConfig = async (params: object) => {
  const localDataPath = await localDataDir();
  const path = await join(localDataPath, "appscheduler");
  const config_file = await join(path, "config.json");
  if (await exists(path)) {
    await writeTextFile(config_file, JSON.stringify(params));
  } else {
    await createDir(path);
    await writeTextFile(config_file, JSON.stringify(params));
  }
};
