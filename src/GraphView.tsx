import { invoke } from "@tauri-apps/api/core";
import type { Color } from "@tauri-apps/api/webview";
import { useEffect, useRef } from "react";

type DataPoint = {
    values: number[];
    distance: number;
};

type TelemetryContext = {
    max_value: number;
    unit: string;
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

function isDataPoint(value: unknown): value is DataPoint {
    if (!value || typeof value !== "object") {
        return false;
    }

    const candidate = value as { values?: unknown; distance?: unknown };
    return (
        Array.isArray(candidate.values) &&
        candidate.values.every(entry => typeof entry === "number" && Number.isFinite(entry)) &&
        typeof candidate.distance === "number" &&
        Number.isFinite(candidate.distance)
    );
}

function isTelemetryContext(value: unknown): value is TelemetryContext {
    if (!value || typeof value !== "object") {
        return false;
    }

    const candidate = value as { max_value?: unknown; unit?: unknown };
    return (
        typeof candidate.max_value === "number" &&
        Number.isFinite(candidate.max_value) &&
        typeof candidate.unit === "string"
    );
}

async function getDataPoint(type: string, carNum: number): Promise<DataPoint> {
    try {
        const raw = await invoke<unknown>("get_values", { teleType: type, carNum });

        if (!isDataPoint(raw)) {
            throw new Error("Telemetry payload is not a valid DataPoint.");
        }

        return raw;
    } catch (error) {
        if (error instanceof Error) {
            throw new Error(`Failed to fetch telemetry: ${error.message}`);
        }

        throw new Error("Failed to fetch telemetry: unknown error.");
    }
}

async function getContext(type: string, carNum: number): Promise<TelemetryContext> {
    try {
        const raw = await invoke<unknown>("get_context", { teleType: type, carNum });

        if (!isTelemetryContext(raw)) {
            throw new Error("Telemetry payload is not a valid DataPoint.");
        }

        return raw;
    } catch (error) {
        if (error instanceof Error) {
            throw new Error(`Failed to fetch telemetry: ${error.message}`);
        }

        throw new Error("Failed to fetch telemetry: unknown error.");
    }
}

async function getLap(type: string, carNum: number): Promise<number> {
    try {
        const raw = await invoke<number>("get_lap", { teleType: type, carNum });

        if (!raw) {
            throw new Error("Telemetry payload is not a valid DataPoint.");
        }

        return raw;
    } catch (error) {
        if (error instanceof Error) {
            throw new Error(`Failed to fetch telemetry: ${error.message}`);
        }

        throw new Error("Failed to fetch telemetry: unknown error.");
    }
}

async function isLastBest(type: string, carNum: number): Promise<boolean> {
    try {
        const raw = await invoke<boolean>("is_last_best", { teleType: type, carNum });

        return raw;
    } catch (error) {
        if (error instanceof Error) {
            throw new Error(`Failed to fetch telemetry: ${error.message}`);
        }

        throw new Error("Failed to fetch telemetry: unknown error.");
    }
}

function render(
    canvas: HTMLCanvasElement | null,
    currentLap: DataPoint[],
    referenceLap: DataPoint[],
    style: GraphViewStyle,
): void {
    const width = canvas?.clientWidth ?? canvas?.width ?? 0;
    const height = canvas?.clientHeight ?? canvas?.height ?? 0;
    const margin = height * 0.15;
    const drawableTop = margin;
    const drawableBottom = height - margin;
    const drawableHeight = drawableBottom - drawableTop;

    function renderLap(canvas: HTMLCanvasElement, lap: DataPoint[], Color: Color) {
        const ctx = canvas.getContext("2d");
        if (!ctx) return;

        ctx.strokeStyle = `${Color}`;
        ctx.lineWidth = 1.5;

        for (let i = 0; i < lap[0].values.length; i++) {
            ctx.beginPath();

            lap.forEach((dp, j) => {
                const dist = dp.distance;
                const val = dp.values[i];
                const x = dist * width;
                const y = drawableBottom - (val ?? 0) * drawableHeight;

                if (j === 0) {
                    ctx.moveTo(x, y);
                    return;
                }

                ctx.lineTo(x, y);
            });

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
    ) {
        const ctx = canvas.getContext("2d");
        if (!ctx) return;

        ctx.clearRect(0, 0, width, height);

        ctx.fillStyle = `${baseColor.toString()}80`; // I am sincearly sorry about this :)
        ctx.fillRect(0, 0, width, height);

        const segments = Math.max(1, nLines) - 1;

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

    renderBackground(canvas, style.baseColor, style.nLines, style.maxValue, style.unit, style.graphName);

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

function GraphView({ baseColor, nLines, type, carNum, graphName }: GraphViewProps) {
    const canvasRef = useRef<HTMLCanvasElement | null>(null);
    const currentLapRef = useRef<DataPoint[]>([]);
    const referenceLapRef = useRef<DataPoint[]>([]);
    const lapNum = useRef<number>(0);
    const wrapperRef = useRef<HTMLDivElement | null>(null);
    const style = useRef<GraphViewStyle>({
        baseColor: baseColor,
        graphName: graphName,
        maxValue: 0,
        nLines: nLines,
        unit: "",
    });

    useEffect(() => {
        const wrapper = wrapperRef.current;
        const canvas = canvasRef.current;
        let active = true;
        void (async () => {
            const telemetryContext: TelemetryContext = await getContext(type, carNum);
            style.current = {
                baseColor: style.current.baseColor,
                graphName: style.current.graphName,
                maxValue: telemetryContext.max_value,
                nLines: style.current.nLines,
                unit: telemetryContext.unit,
            };
            console.log("Updated Context");
        })();

        if (!wrapper || !canvas) return;

        const resizeCanvas = () => {
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

            render(canvas, currentLapRef.current, referenceLapRef.current, style.current);
        };

        const resizeObserver = new ResizeObserver(() => resizeCanvas());

        resizeObserver.observe(wrapper);
        resizeCanvas();

        const interval = setInterval(() => {
            void (async () => {
                try {
                    const dataPoint: DataPoint = await getDataPoint(type, carNum);
                    const newLap: number = await getLap(type, carNum);
                    if (newLap != lapNum.current) {
                        lapNum.current = newLap;
                        if (await isLastBest(type, carNum)) {
                            referenceLapRef.current = currentLapRef.current;
                            currentLapRef.current = [];
                        }
                    }
                    if (!active) {
                        return;
                    }
                    if (dataPoint != currentLapRef.current[currentLapRef.current.length - 1])
                        currentLapRef.current = [...currentLapRef.current, dataPoint];

                    render(canvas, currentLapRef.current, referenceLapRef.current, style.current);
                } catch (e) {
                    render(
                        canvas,
                        [],
                        [
                            // { distance: 0.42, values: [0, 0.4] },
                            // { distance: 0.8, values: [1, 0.6] },
                        ],
                        style.current,
                    ); // Can be used for testign purposes, but in prod this will handle rendering of the empty state.
                    console.log(e);
                }
            })();
        }, 16);

        return () => {
            active = false;
            clearInterval(interval);
            resizeObserver.disconnect();
        };
    }, []);

    return (
        <div ref={wrapperRef} className="w-full h-full">
            <canvas ref={canvasRef} className="block w-full h-full rounded-xl" />
        </div>
    );
}

export default GraphView;
