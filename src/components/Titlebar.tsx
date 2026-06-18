import { getCurrentWindow } from "@tauri-apps/api/window";

export default function Titlebar() {
  const appWindow = getCurrentWindow();
  return (
    <div
      className="w-full h-7 flex justify-end items-center pr-4 gap-3"
      data-tauri-drag-region
    >
      <button
        onClick={() => appWindow.minimize()}
        className="rounded-full w-4 h-4 bg-yellow-400"
      ></button>
      <button
        onClick={() => appWindow.maximize()}
        className="rounded-full w-4 h-4 bg-green-600"
      ></button>
      <button
        onClick={() => appWindow.close()}
        className="rounded-full w-4 h-4 bg-red-600"
      ></button>
    </div>
  );
}
