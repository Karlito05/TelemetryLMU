import { useContext, useEffect, useState } from "react";
import Sidebar from "./sidebar/sidebar.tsx";
import PlayerView from "./playerView/playerView.tsx";
import { SettingsContext } from "@/App.tsx";
import { invoke } from "@tauri-apps/api/core";

type ChampDriverData = {
  name: string;
  car: string;
  car_class: string;
  id: number;
};

export default function Championships() {
  const [curDriverId, setCurDriverId] = useState(0);
  const s = useContext(SettingsContext);

  useEffect(() => {
    invoke<ChampDriverData[]>("get_champ_drivers").then((v) => {
      setCurDriverId(
        v.find((d) => {
          return d.name == s.gameName;
        })?.id || 0,
      );
    });
  });

  return (
    <div className="flex gap-2 h-full w-full">
      {/* <Sidebar curDriverId={curDriverId} setCurDriverId={setCurDriverId} /> */}
      <PlayerView curDriverId={curDriverId} />
    </div>
  );
}
