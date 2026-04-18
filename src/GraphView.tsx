import { invoke, Channel } from "@tauri-apps/api/core";
import type { Color } from "@tauri-apps/api/webview";
import { useEffect, useRef } from "react";

type DataPoint = {
  values: number[];
  distance: number;
};

type GraphViewStyle = {
  baseColor: Color;
  maxValue: number;
  unit: string;
  graphName: string;
  nLines: number;
};

type GraphViewProps = {
  baseColor: Color;
  nLines: number;
  type: string;
  carNum: number;
  graphName: string;
};

type LapEvent =
  | {
      event: "renderingData";
      data: {
        maxValue: number;
        unit: string;
      };
    }
  | {
      event: "lapDataPoint";
      data: {
        values: number[];
        distance: number;
      };
    }
  | {
      event: "lapFinished";
      data: {
        wasBest: boolean;
      };
    };

function render(
  canvas: HTMLCanvasElement | null,
  currentLap: DataPoint[],
  referenceLap: DataPoint[],
  style: GraphViewStyle,
  type: string,
): void {
  const width = canvas?.clientWidth ?? canvas?.width ?? 0;
  const height = canvas?.clientHeight ?? canvas?.height ?? 0;
  const margin = height * 0.15;
  const drawableTop = margin;
  const drawableBottom = height - margin;
  const drawableHeight = drawableBottom - drawableTop;

  function renderLap(canvas: HTMLCanvasElement, lap: DataPoint[], Color: Color) {
    const ctx = canvas.getContext("2d");
    if (!ctx || lap.length === 0) return;

    const firstValid = lap.find((dp) => dp !== undefined);
    if (!firstValid) return;

    ctx.strokeStyle = `${Color}`;
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

  function renderBackground(
    canvas: HTMLCanvasElement,
    baseColor: Color,
    nLines: number,
    maxVal: number,
    unit: string,
    graphName: string,
    type: string,
  ) {
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    ctx.clearRect(0, 0, width, height);

    ctx.fillStyle = `${baseColor.toString()}80`; // I am sincearly sorry about this :)
    ctx.fillRect(0, 0, width, height);

    const segments = Math.max(1, nLines) - 1;

    if (type == "delta")
      for (let i = 0; i <= segments; i++) {
        const t = i / segments;
        const y = drawableBottom - t * drawableHeight;

        ctx.strokeStyle = "#FFFFFF40";
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(width, y);
        ctx.stroke();

        ctx.font = "14px 'inter', sans-serif";
        ctx.fillStyle = "#FFFFFF70";
        ctx.fillText(`${-(maxVal / 2 - maxVal * t)} ${unit}`, 0, y - 2);
      }
    else if (maxVal == 1)
      for (let i = 0; i <= segments; i++) {
        const t = i / segments;
        const y = drawableBottom - t * drawableHeight;

        ctx.strokeStyle = "#FFFFFF40";
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(width, y);
        ctx.stroke();

        ctx.font = "14px 'inter', sans-serif";
        ctx.fillStyle = "#FFFFFF70";
        ctx.fillText(`${maxVal * t * 100} ${unit}`, 0, y - 2);
      }
    else
      for (let i = 0; i <= segments; i++) {
        const t = i / segments;
        const y = drawableBottom - t * drawableHeight;

        ctx.strokeStyle = "#FFFFFF40";
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(width, y);
        ctx.stroke();

        ctx.font = "14px 'inter', sans-serif";
        ctx.fillStyle = "#FFFFFF70";
        ctx.fillText(`${maxVal * t} ${unit}`, 0, y - 2);
      }

    ctx.save();
    ctx.font = "16px 'Days One', sans-serif";
    ctx.fillStyle = baseColor.toString();
    ctx.textBaseline = "middle";
    ctx.fillText(graphName, 10, canvas.height - drawableTop / 2);
    ctx.restore();
  }

  if (!canvas) return;
  void referenceLap;

  const ctx = canvas.getContext("2d");
  if (!ctx) return;

  ctx.clearRect(0, 0, width, height);

  renderBackground(
    canvas,
    style.baseColor,
    style.nLines,
    style.maxValue,
    style.unit,
    style.graphName,
    type,
  );

  if (currentLap.length === 0 && referenceLap.length === 0) {
    ctx.font = "20px 'Days One', sans-serif";
    ctx.fillStyle = "#fff";
    ctx.fillText("No data available", width / 2 - 70, height / 2);
    return;
  } else {
    if (currentLap.length != 0) renderLap(canvas, currentLap, style.baseColor);
    if (referenceLap.length != 0) renderLap(canvas, referenceLap, `${style.baseColor}80`);
  }
}

function resizeCanvas(
  wrapper: HTMLDivElement,
  canvas: HTMLCanvasElement,
  currentLap: DataPoint[],
  referenceLap: DataPoint[],
  style: GraphViewStyle,
  type: string,
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

  // Keep drawing commands in CSS pixels while using a high-DPI buffer.
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

  render(canvas, currentLap, referenceLap, style, type);
}

const RESOLUTION = 10000;

function GraphView({ baseColor, nLines, type, carNum, graphName }: GraphViewProps) {
  const wrapperRef = useRef<HTMLDivElement | null>(null);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const style = useRef<GraphViewStyle>({
    baseColor: baseColor,
    maxValue: 0,
    unit: "",
    graphName: graphName,
    nLines: nLines,
  });
  const curLapRef = useRef<DataPoint[]>(new Array(RESOLUTION));
  const refLapRef = useRef<DataPoint[]>([]);

  useEffect(() => {
    const onEvent = new Channel<LapEvent>();

    onEvent.onmessage = (message) => {
      switch (message.event) {
        case "renderingData":
          style.current.maxValue = message.data.maxValue;
          style.current.unit = message.data.unit;
          break;
        case "lapDataPoint":
          const { distance, values } = message.data;
          const canvas = canvasRef.current;
          if (!canvas) return;

          const index = Math.floor(distance * (RESOLUTION - 1));

          curLapRef.current[index] = {
            values: [...values],
            distance: distance,
          };
          break;
        case "lapFinished":
          if (message.data.wasBest || refLapRef.current.filter(Boolean).length < 500) {
            refLapRef.current = curLapRef.current;
          }
          curLapRef.current = new Array(RESOLUTION);
          break;
      }

      render(canvasRef.current, curLapRef.current, refLapRef.current, style.current, type);
    };

    invoke("lap_data_subscribe", { teleType: type, carNum: carNum, onEvent: onEvent });

    let wrapper = wrapperRef.current;
    let canvas = canvasRef.current;

    if (!wrapper || !canvas) return;

    const resizeObserver = new ResizeObserver(() =>
      resizeCanvas(wrapper, canvas, curLapRef.current, refLapRef.current, style.current, type),
    );

    resizeObserver.observe(wrapper);
    resizeCanvas(wrapper, canvas, curLapRef.current, refLapRef.current, style.current, type);
  }, []);

  return (
    <div ref={wrapperRef} className="w-full h-full">
      <canvas ref={canvasRef} className="block w-full h-full rounded-xl" />
    </div>
  );
}

export default GraphView;
