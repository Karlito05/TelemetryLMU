import PlayerCard, { CarClass } from "./playerCard";

export default function Sidebar() {
  return (
    <div className="min-w-[300px] max-w-[350px] w-2/10 rounded-[24px] overflow-hidden gap-1.5 flex flex-col">
      <PlayerCard name="Karel Lukes" carClass={CarClass.GT3} car="Porsche 911 GT3 R" />
      <PlayerCard name="Karel Lukes" carClass={CarClass.GT3} car="Porsche 911 GT3 R" />
      <PlayerCard name="Karel Lukes" carClass={CarClass.GT3} car="Porsche 911 GT3 R" />
    </div>
  );
}
