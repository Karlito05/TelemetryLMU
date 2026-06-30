import { invoke, Channel } from "@tauri-apps/api/core";
import type { Color } from "@tauri-apps/api/webview";
import { useEffect, useRef } from "react";

export type DataPoint = {
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
  componentStyle?: React.CSSProperties;
  refLap?: DataPoint[];
};

type LapEvent =
  | {
      event: "renderingData";
      data: {
        maxValue: number;
        unit: string;
        id: string;
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

  function renderLap(
    canvas: HTMLCanvasElement,
    lap: DataPoint[],
    Color: Color,
  ) {
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
        ctx.fillText(`${Math.trunc(maxVal * t * 100)} ${unit}`, 0, y - 2);
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
        ctx.fillText(`${Math.trunc(maxVal * t)} ${unit}`, 0, y - 2);
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
    if (referenceLap.length != 0)
      renderLap(canvas, referenceLap, `${style.baseColor}80`);
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

const RESOLUTION = 2000;

function downsampleByDistance(
  data: DataPoint[],
  targetCount: number,
): DataPoint[] {
  if (data.length === 0) return [];
  if (data.length <= targetCount) return data;

  const step = 1 / (targetCount - 1);
  const result: DataPoint[] = [];

  let searchIndex = 0;
  for (let i = 0; i < targetCount; i++) {
    const targetDist = i * step;

    while (
      searchIndex < data.length - 1 &&
      Math.abs(data[searchIndex + 1].distance - targetDist) <=
        Math.abs(data[searchIndex].distance - targetDist)
    ) {
      searchIndex++;
    }

    result.push(data[searchIndex]);
  }

  return result;
}

function GraphView({
  baseColor,
  nLines,
  carNum,
  type,
  graphName,
  componentStyle,
  refLap,
}: GraphViewProps) {
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
  const refLapRef = useRef<DataPoint[]>(refLap ? refLap : []);
  const id = useRef("");

  useEffect(() => {
    if (refLap) {
      refLapRef.current = downsampleByDistance(refLap, RESOLUTION);
    } else {
      refLapRef.current = [];
    }
  }, [refLap]);

  useEffect(() => {
    const onEvent = new Channel<LapEvent>();
    let isActive = true;

    onEvent.onmessage = (message) => {
      if (!isActive) return;

      switch (message.event) {
        case "renderingData":
          style.current.maxValue = message.data.maxValue;
          style.current.unit = message.data.unit;
          id.current = message.data.id;
          break;
        case "lapDataPoint":
          if (!canvasRef.current) return;

          curLapRef.current[
            Math.floor(message.data.distance * (RESOLUTION - 1))
          ] = {
            values: [...message.data.values],
            distance: message.data.distance,
          };
          break;
        case "lapFinished":
          if (
            (message.data.wasBest ||
              refLapRef.current.filter(Boolean).length < 500) &&
            type != "delta" &&
            !refLap
          ) {
            refLapRef.current = curLapRef.current;
          }
          curLapRef.current = new Array(RESOLUTION);
          break;
      }

      render(
        canvasRef.current,
        curLapRef.current,
        refLapRef.current,
        style.current,
        type,
      );
    };

    invoke("lap_data_subscribe", {
      teleType: type,
      carNum: carNum,
      onEvent: onEvent,
    });

    const wrapper = wrapperRef.current;
    const canvas = canvasRef.current;

    if (!wrapper || !canvas) return;

    const resizeObserver = new ResizeObserver(() =>
      resizeCanvas(
        wrapper,
        canvas,
        curLapRef.current,
        refLapRef.current,
        style.current,
        type,
      ),
    );

    resizeObserver.observe(wrapper);
    resizeCanvas(
      wrapper,
      canvas,
      curLapRef.current,
      refLapRef.current,
      style.current,
      type,
    );

    return () => {
      isActive = false;
      // This is a fix for the strict mode in react. This really should't happen in prod cos its basically a race condition
      // TODO: This fix
      if (id.current == "") {
        console.log(`ID is not set. Failing back to id: ${type}-${carNum}`);
        invoke("lap_data_unsubscribe", { id: `${type}-${carNum}` });
      } else {
        console.log(`Invoked lap_data_unsubscribe with id: ${id.current}`);
        invoke("lap_data_unsubscribe", { id: id.current });
      }
      curLapRef.current = new Array(RESOLUTION);
      refLapRef.current = [];

      const ctx = canvasRef.current?.getContext("2d");
      if (ctx && canvasRef.current) {
        ctx.clearRect(0, 0, canvasRef.current.width, canvasRef.current.height);
      }
    };
  }, [baseColor, nLines, carNum, type, graphName]);

  return (
    <div ref={wrapperRef} className="w-full h-full">
      <canvas
        ref={canvasRef}
        className="block w-full h-full"
        style={{ ...componentStyle }}
      />
    </div>
  );
}

export default GraphView;
