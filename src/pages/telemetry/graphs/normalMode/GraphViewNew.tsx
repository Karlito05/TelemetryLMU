import { Application, extend, useApplication } from "@pixi/react";
import { Graphics, Text } from "pixi.js";
import { useRef } from "react";

extend({ Graphics, Text });

export type DataPoint = {
  values: number[];
  distance: number;
};

export default function GraphViewNew({
  style,
  currentLap,
  referenceLap,
  componentStyle,
  telemetryInfo,
}: {
  style?: { color: string; gridlines: number };
  currentLap?: DataPoint[];
  referenceLap?: DataPoint[];
  componentStyle?: React.CSSProperties;
  telemetryInfo?: { unit: string; type: string; maxVal: number };
}) {
  const parentRef = useRef<HTMLDivElement>(null);
  return (
    <div ref={parentRef} style={componentStyle} className="w-full h-full overflow-hidden">
      <Application resizeTo={parentRef}>
        <Background
          telemetryInfo={telemetryInfo ?? { maxVal: 0, type: "N/A", unit: "N/A" }}
          style={style ?? { color: "#FFFFFF", gridlines: 3 }}
        />
        <DrawLap lap={currentLap ?? []} color={style ? style.color : "#FFFFFF"} alpha={1} />
        <DrawLap lap={referenceLap ?? []} color={style ? style.color : "#FFFFFF"} alpha={0.25} />
      </Application>
    </div>
  );
}

function DrawLap({ lap, color, alpha }: { lap: DataPoint[]; color: string; alpha: number }) {
  const app = useApplication();
  const width = app.app.renderer.screen.width;
  const height = app.app.renderer.screen.height;
  const margin = height * 0.15;
  const drawableTop = margin;
  const drawableBottom = height - margin;
  const drawableHeight = drawableBottom - drawableTop;
  return (
    <pixiGraphics
      draw={(g) => {
        g.clear();
        // TODO: Fix this outer loop
        //
        // for (let i = 0; i < lap.values.length; i++) {
        g.beginPath();
        let lastDist = -1; // Keep track of the previous point's distance

        for (let j = 0; j < lap.length; j++) {
          const dp = lap[j];
          if (!dp) continue;

          const x = dp.distance * width;
          const y = drawableBottom - (dp.values[0] ?? 0) * drawableHeight;

          if (lastDist === -1 || Math.abs(dp.distance - lastDist) > 0.05) {
            g.moveTo(x, y);
          } else {
            g.lineTo(x, y);
          }

          lastDist = dp.distance;
        }
        g.stroke({ color: color, alpha: alpha, width: 2 });
        // }
      }}
    />
  );
}

function Background({
  style,
  telemetryInfo,
}: {
  style: { color: string; gridlines: number };
  telemetryInfo: { unit: string; type: string; maxVal: number };
}) {
  const app = useApplication();
  const width = app.app.renderer.screen.width;
  const height = app.app.renderer.screen.height;
  const margin = height * 0.15;
  const drawableTop = margin;
  const drawableBottom = height - margin;
  const drawableHeight = drawableBottom - drawableTop;

  return (
    <pixiGraphics
      draw={(g) => {
        g.clear();

        g.rect(0, 0, width, height);
        g.fill({ color: 0x16171c });

        const segments = Math.max(1, style.gridlines) - 1;

        if (telemetryInfo.type == "delta")
          for (let i = 0; i <= segments; i++) {
            const t = i / segments;
            const y = drawableBottom - t * drawableHeight;

            dashedLine(g, 0, y, width, y);
            g.stroke({ color: 0xffffff, alpha: 0.5 });

            // g.font = "14px 'inter', sans-serif";
            // g.fillStyle = "#FFFFFF70";
            // g.fillText(`${-(maxVal / 2 - maxVal * t)} ${unit}`, 0, y - 2);
          }
        else if (telemetryInfo.maxVal == 1)
          for (let i = 0; i <= segments; i++) {
            const t = i / segments;
            const y = drawableBottom - t * drawableHeight;

            dashedLine(g, 0, y, width, y);
            g.stroke({ color: 0xffffff, alpha: 0.5 });

            // g.font = "14px 'inter', sans-serif";
            // g.fillStyle = "#FFFFFF70";
            // g.fillText(`${Math.trunc(maxVal * t * 100)} ${unit}`, 0, y - 2);
          }
        else
          for (let i = 0; i <= segments; i++) {
            const t = i / segments;
            const y = drawableBottom - t * drawableHeight;

            dashedLine(g, 0, y, width, y);
            g.stroke();
            g.stroke({ color: 0xffffff, alpha: 0.5 });

            // g.font = "14px 'inter', sans-serif";
            // g.fillStyle = "#FFFFFF70";
            // g.fillText(`${Math.trunc(maxVal * t)} ${unit}`, 0, y - 2);
          }

        // g.save();
        // g.font = "16px 'Days One', sans-serif";
        // g.fillStyle = baseColor.toString();
        // g.textBaseline = "middle";
        // g.fillText(graphName, 10, canvas.height - drawableTop / 2);
        // g.restore();
      }}
    >
      {Array.from({ length: Math.max(1, style.gridlines) }).map((_, i) => {
        const t = i / (Math.max(1, style.gridlines) - 1);
        const y = drawableBottom - t * drawableHeight;

        if (telemetryInfo.type == "delta")
          return (
            <pixiText
              key={i}
              x={0}
              y={y - 2}
              text={`${-(telemetryInfo.maxVal / 2 - telemetryInfo.maxVal * t)} ${telemetryInfo.unit}`}
              style={{ fill: 0x8a8b8d, fontSize: 14, fontFamily: "inter" }}
              anchor={{ x: 0, y: 1 }}
            />
          );
        else if (telemetryInfo.maxVal == 1)
          return (
            <pixiText
              key={i}
              x={0}
              y={y - 2}
              text={`${Math.trunc(telemetryInfo.maxVal * t * 100)} ${telemetryInfo.unit}`}
              style={{ fill: 0x8a8b8d, fontSize: 14, fontFamily: "inter" }}
              anchor={{ x: 0, y: 1 }}
            />
          );
        else
          return (
            <pixiText
              key={i}
              x={0}
              y={y - 2}
              text={`${Math.trunc(telemetryInfo.maxVal * t)} ${telemetryInfo.unit}`}
              style={{ fill: 0x8a8b8d, fontSize: 14, fontFamily: "inter" }}
              anchor={{ x: 0, y: 1 }}
            />
          );
      })}
      <pixiText
        x={10}
        y={height - drawableTop / 2}
        text={telemetryInfo.type.charAt(0).toUpperCase() + telemetryInfo.type.slice(1)}
        style={{ fill: style.color, fontSize: 18, fontFamily: "Racing Sans One" }}
        anchor={{ x: 0, y: 0.5 }}
      />
    </pixiGraphics>
  );
}

function dashedLine(
  g: Graphics,
  x1: number,
  y1: number,
  x2: number,
  y2: number,
  dash = 8,
  gap = 8,
) {
  const dx = x2 - x1;
  const dy = y2 - y1;
  const length = Math.sqrt(dx * dx + dy * dy);
  const angle = Math.atan2(dy, dx);

  let drawn = 0;

  g.moveTo(x1, y1);

  while (drawn < length) {
    const xStart = x1 + Math.cos(angle) * drawn;
    const yStart = y1 + Math.sin(angle) * drawn;

    drawn += dash;

    if (drawn > length) drawn = length;

    const xEnd = x1 + Math.cos(angle) * drawn;
    const yEnd = y1 + Math.sin(angle) * drawn;

    g.moveTo(xStart, yStart);
    g.lineTo(xEnd, yEnd);

    drawn += gap;
  }
}
