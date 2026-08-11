import { useState } from "react";
import Sidebar from "./sidebar/sidebar.tsx";
import PlayerView from "./playerView/playerView.tsx";

export default function Championships() {
  const [curDriverId, setCurDriverId] = useState(0);
  return (
    <div className="flex gap-2 h-full w-full">
      <Sidebar curDriverId={curDriverId} setCurDriverId={setCurDriverId} />
      <PlayerView curDriverId={curDriverId} />
    </div>
  );
}
