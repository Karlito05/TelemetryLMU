import { IconWheel } from "@tabler/icons-react";

export type Tires = {
  fl: Tire;
  fr: Tire;
  rl: Tire;
  rr: Tire;
};

type Tire = {
  health: number; //0-1 percentage
  inside_temp: number;
  outside_temp: number;
  brake_temp: number;
};

export default function TiresCard({ tires }: { tires: Tires }) {
  return (
    <div className="bg-[#FFFFFF0C] rounded-[24px] p-4 flex-1">
      <div className="text-[#FFFFFF60] text-[14px] font-[Barlow_Condesed] flex gap-1 items-center">
        <IconWheel size={12} /> TIRES
      </div>
      <div className="flex gap-2 flex-col">
        <div className="flex gap-2">
          <Tire
            tire={{
              tire: tires.fl,
              name: "FL",
            }}
          />
          <Tire
            tire={{
              tire: tires.fr,
              name: "FR",
            }}
          />
        </div>
        <div className="flex gap-2">
          <Tire
            tire={{
              tire: tires.rl,
              name: "RL",
            }}
          />
          <Tire
            tire={{
              tire: tires.rr,
              name: "RR",
            }}
          />
        </div>
      </div>
    </div>
  );
}

function Tire({ tire }: { tire: { tire: Tire; name: string } }) {
  return (
    <div className="bg-[#FFFFFF10] rounded-[8px] p-1 flex-1">
      <div className="flex justify-between font-[Barlow_Condensed] text-[14px]">
        {tire.name}
        <div
          className="flex text-[#00FF00] bg-[#00FF0018] rounded-[2px] px-1 items-center justify-center font-[Inter]"
          style={{
            color: interpolateColor(tire.tire.health, 0, 1, "#FF0000", "#00FF00"),
            backgroundColor: `${interpolateColor(tire.tire.health, 0, 1, "#FF0000", "#00FF00")}18`,
          }}
        >
          {tire.tire.health * 100}
          {"%"}
        </div>
      </div>
      <div className="flex gap-1">
        <div className="bg-[#FFFFFF18] w-3/100  rounded-full items-end flex">
          <div
            className="w-full rounded-full"
            style={{
              height:
                Math.ceil((tire.tire.inside_temp + tire.tire.outside_temp) / 2) > 100
                  ? "100%"
                  : Math.ceil((tire.tire.inside_temp + tire.tire.outside_temp) / 2).toString() +
                    "%",
              backgroundColor: interpolateColor3(
                Math.ceil((tire.tire.inside_temp + tire.tire.outside_temp) / 2),
                40,
                80,
                100,
                "#0000FF",
                "#00FF00",
                "#FF0000",
              ),
            }}
          />
        </div>
        <div className="font-[Jet_Brains_Mono] flex-1">
          <div className="flex justify-between items-center">
            Inside
            <div
              style={{
                color: interpolateColor3(
                  tire.tire.inside_temp,
                  30,
                  80,
                  100,
                  "#0000FF",
                  "#00FF00",
                  "#FF0000",
                ),
              }}
            >
              {tire.tire.inside_temp}
              {"°C"}
            </div>
          </div>
          <div className="flex justify-between items-center">
            Outside
            <div
              style={{
                color: interpolateColor3(
                  tire.tire.outside_temp,
                  30,
                  80,
                  100,
                  "#0000FF",
                  "#00FF00",
                  "#FF0000",
                ),
              }}
            >
              {tire.tire.outside_temp}
              {"°C"}
            </div>
          </div>
          <div className="flex justify-between items-center">
            Brake
            <div
              style={{
                color: interpolateColor3(
                  tire.tire.brake_temp,
                  30,
                  120,
                  800,
                  "#0000FF",
                  "#00FF00",
                  "#FF0000",
                ),
              }}
            >
              {tire.tire.brake_temp}
              {"°C"}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

function interpolateColor(
  value: number,
  min: number,
  max: number,
  startColor: string,
  endColor: string,
): string {
  // Clamp value to range
  const t = Math.max(0, Math.min(1, (value - min) / (max - min)));

  const start = hexToRgb(startColor);
  const end = hexToRgb(endColor);

  const r = Math.round(start.r + (end.r - start.r) * t);
  const g = Math.round(start.g + (end.g - start.g) * t);
  const b = Math.round(start.b + (end.b - start.b) * t);

  return rgbToHex(r, g, b);
}
function interpolateColor3(
  value: number,
  min: number,
  mid: number,
  max: number,
  startColor: string,
  midColor: string,
  endColor: string,
): string {
  value = Math.max(min, Math.min(max, value));

  if (value <= mid) {
    const t = (value - min) / (mid - min);
    return interpolateBetween(startColor, midColor, t);
  }

  const t = (value - mid) / (max - mid);
  return interpolateBetween(midColor, endColor, t);
}

function interpolateBetween(colorA: string, colorB: string, t: number): string {
  const a = hexToRgb(colorA);
  const b = hexToRgb(colorB);

  const r = Math.round(a.r + (b.r - a.r) * t);
  const g = Math.round(a.g + (b.g - a.g) * t);
  const bVal = Math.round(a.b + (b.b - a.b) * t);

  return rgbToHex(r, g, bVal);
}

function hexToRgb(hex: string) {
  const clean = hex.replace("#", "");
  return {
    r: parseInt(clean.slice(0, 2), 16),
    g: parseInt(clean.slice(2, 4), 16),
    b: parseInt(clean.slice(4, 6), 16),
  };
}

function rgbToHex(r: number, g: number, b: number): string {
  return "#" + [r, g, b].map((x) => x.toString(16).padStart(2, "0")).join("");
}
