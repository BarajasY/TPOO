import { open } from '@tauri-apps/api/shell';
import { appLocalDataDir } from "@tauri-apps/api/path";
import './OpenFolder.scss';

const OpenFolder = () => {

  const openFolder = async () => {
    const path = `${await appLocalDataDir()}/excel/`
    open(path)
  }

  return (
    <div class="open-folder">
      <button onclick={() => openFolder()}>Abrir Carpeta</button>
    </div>
  )
}

export default OpenFolder
