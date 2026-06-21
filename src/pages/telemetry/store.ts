import { load } from "@tauri-apps/plugin-store";

export type Layouts = {
  name: string;
  scales: string[];
  graphData: GraphViewData[];
};

export type GraphViewData = {
  baseColor: string;
  nLines: number;
  type: GraphViewType;
};

export enum GraphViewType {
  Throttle = "throttle",
  Brake = "brake",
  Rpm = "rpm",
  Delta = "delta",
  Speed = "speed",
}

export async function setLayouts(value: Layouts[]): Promise<void> {
  const store = await load("layouts.json");
  await store.set("layouts", value);
  await store.save();
}

export async function getLayouts(): Promise<Layouts[] | undefined> {
  const store = await load("layouts.json");
  const val = await store.get<Layouts[]>("layouts");
  return val;
}
