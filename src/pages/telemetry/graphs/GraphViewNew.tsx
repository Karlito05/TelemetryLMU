import { DataPoint } from "./GraphView";
import { useEffect, useRef } from "react";

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
  const wrapperRef = useRef<HTMLDivElement | null>(null);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  useEffect(() => {
    // Set up refs
    const canvas = canvasRef.current;
    const wrapper = wrapperRef.current;
    if (!canvas || !wrapper) return;

    // Set up ResizeObserver
    const resizeObserver = new ResizeObserver(() =>
      resizeCanvas(
        wrapper,
        canvas,
        currentLap ? currentLap : [],
        referenceLap ? referenceLap : [],
        telemetryInfo ? telemetryInfo : { unit: "N/A", type: "N/A", maxVal: 0 },
        style ? style : { color: "#138DF1", gridlines: 3 },
      ),
    );
    resizeObserver.observe(wrapper);
    resizeCanvas(
      wrapper,
      canvas,
      currentLap ? currentLap : [],
      referenceLap ? referenceLap : [],
      telemetryInfo ? telemetryInfo : { unit: "N/A", type: "N/A", maxVal: 0 },
      style ? style : { color: "#138DF1", gridlines: 3 },
    );

    // Render
    render(
      canvas,
      currentLap ? currentLap : [],
      referenceLap ? referenceLap : [],
      telemetryInfo ? telemetryInfo : { unit: "N/A", type: "N/A", maxVal: 0 },
      style ? style : { color: "#138DF1", gridlines: 3 },
    );
  }, [style, currentLap, referenceLap]);

  return (
    <div ref={wrapperRef} className="w-full h-full">
      <canvas ref={canvasRef} className="block w-full h-full" style={{ ...componentStyle }} />
    </div>
  );
}

function render(
  canvas: HTMLCanvasElement,
  currentLap: DataPoint[],
  referenceLap: DataPoint[],
  telemetryInfo: { unit: string; type: string; maxVal: number },
  style: { color: string; gridlines: number },
) {
  renderBackground(
    canvas,
    style.color,
    style.gridlines,
    telemetryInfo.maxVal,
    telemetryInfo.unit,
    telemetryInfo.type,
  );

  renderLap(canvas, referenceLap, `${style.color}80`);
  renderLap(canvas, currentLap, style.color);
}

function renderBackground(
  canvas: HTMLCanvasElement,
  baseColor: string,
  nLines: number,
  maxVal: number,
  unit: string,
  type: string,
) {
  const width = canvas.clientWidth ?? canvas.width ?? 0;
  const height = canvas.clientHeight ?? canvas.height ?? 0;
  const margin = height * 0.15;
  const drawableTop = margin;
  const drawableBottom = height - margin;
  const drawableHeight = drawableBottom - drawableTop;

  const ctx = canvas.getContext("2d");
  if (!ctx) return;

  ctx.clearRect(0, 0, width, height);

  ctx.fillStyle = `#16171C`;
  ctx.fillRect(0, 0, width, height);

  const segments = Math.max(1, nLines) - 1;

  if (type == "delta")
    for (let i = 0; i <= segments; i++) {
      const t = i / segments;
      const y = drawableBottom - t * drawableHeight;

      ctx.setLineDash([8, 8]);

      ctx.strokeStyle = "#FFFFFF40";
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(width, y);
      ctx.stroke();

      ctx.font = "14px 'inter', sans-serif";
      ctx.fillStyle = "#FFFFFF70";
      ctx.fillText(`${-(maxVal / 2 - maxVal * t)} ${unit}`, 0, y - 8);
    }
  else if (maxVal == 1)
    for (let i = 0; i <= segments; i++) {
      const t = i / segments;
      const y = drawableBottom - t * drawableHeight;

      ctx.setLineDash([8, 8]);

      ctx.strokeStyle = "#FFFFFF40";
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(width, y);
      ctx.stroke();

      ctx.font = "14px 'inter', sans-serif";
      ctx.fillStyle = "#FFFFFF70";
      ctx.fillText(`${Math.trunc(maxVal * t * 100)} ${unit}`, 0, y - 8);
    }
  else
    for (let i = 0; i <= segments; i++) {
      const t = i / segments;
      const y = drawableBottom - t * drawableHeight;

      ctx.setLineDash([8, 8]);

      ctx.strokeStyle = "#FFFFFF40";
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(width, y);
      ctx.stroke();

      ctx.font = "14px 'inter', sans-serif";
      ctx.fillStyle = "#FFFFFF70";
      ctx.fillText(`${Math.trunc(maxVal * t)} ${unit}`, 0, y - 8);
    }

  ctx.save();
  ctx.font = "16px 'Racing Sans One', sans-serif";
  ctx.fillStyle = baseColor;
  ctx.textBaseline = "middle";
  ctx.fillText(
    String(type).charAt(0).toUpperCase() + String(type).slice(1),
    drawableTop / 2,
    canvas.height - drawableTop / 2,
  );
  ctx.restore();
}

function renderLap(canvas: HTMLCanvasElement, lap: DataPoint[], color: string) {
  const width = canvas.clientWidth ?? canvas.width ?? 0;
  const height = canvas.clientHeight ?? canvas.height ?? 0;
  const margin = height * 0.15;
  const drawableTop = margin;
  const drawableBottom = height - margin;
  const drawableHeight = drawableBottom - drawableTop;

  const ctx = canvas.getContext("2d");
  if (!ctx || lap.length === 0) return;

  const firstValid = lap.find((dp) => dp !== undefined);
  if (!firstValid) return;

  ctx.strokeStyle = color;
  ctx.lineWidth = 1.5;

  for (let i = 0; i < firstValid.values.length; i++) {
    ctx.beginPath();
    let lastDist = -1; // Keep track of the previous point's distance

    for (let j = 0; j < lap.length; j++) {
      const dp = lap[j];
      if (!dp) continue;

      const x = dp.distance * width;
      const y = drawableBottom - (dp.values[i] ?? 0) * drawableHeight;

      if (lastDist === -1 || Math.abs(dp.distance - lastDist) > 0.05) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }

      lastDist = dp.distance;
    }
    ctx.stroke();
  }
}

function resizeCanvas(
  wrapper: HTMLDivElement,
  canvas: HTMLCanvasElement,
  currentLap: DataPoint[],
  referenceLap: DataPoint[],
  telemetryInfo: { unit: string; type: string; maxVal: number },
  style: { color: string; gridlines: number },
) {
  const dpr = window.devicePixelRatio || 1;
  const cssWidth = wrapper.clientWidth;
  const cssHeight = wrapper.clientHeight;

  canvas.style.width = `${cssWidth}px`;
  canvas.style.height = `${cssHeight}px`;

  canvas.width = Math.floor(cssWidth * dpr);
  canvas.height = Math.floor(cssHeight * dpr);

  const ctx = canvas.getContext("2d");
  if (!ctx) return;

  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

  render(canvas, currentLap, referenceLap, telemetryInfo, style);
}
