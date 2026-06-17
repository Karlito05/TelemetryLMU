import { useEffect, useState, useContext } from "react";
import { invoke } from "@tauri-apps/api/core";
import { TelemetryContext } from "../../Telemetry";
import { MenuProps } from "antd";
import DriverLayoutWidget from "./DriverLayoutsWidget";

export default function NormalLayout() {
  const [drivers, setDrivers] = useState<MenuProps["items"]>([]);
  const [curDriver, setCurDriver] = useState<string>("");
  const c = useContext(TelemetryContext);

  useEffect(() => {
    invoke<Driver[]>("get_drivers").then((v) => {
      try {
        setCurDriver(v[0].name);
      } catch (e) {
        console.error("No drivers found. The game is probably not runing.");
        setCurDriver("N/A");
      }
      const items: MenuProps["items"] = v.map((driver) => ({
        label: driver.name,
        key: String(driver.index),
      }));

      setDrivers(items);
    });
  }, []);
  return (
    <>
      <DriverLayoutWidget
        drivers={drivers}
        layouts={layouts}
        onDriverSelect={(key, driverName) => {
          const carNum = Number(key);
          setCurDriver(driverName);
          console.log("Set car num ", carNum);
          c.setCurDriverNum(carNum);
        }}
        onLayoutSelect={(key) => {
          if (key == "edit") {
            c.setEditMode(true);
          }
        }}
        curDriver={curDriver}
      />
    </>
  );
}

type Driver = {
  index: number;
  name: string;
};

//TODO: Factor this out
const layouts: MenuProps["items"] = [
  { type: "divider" },
  {
    key: "edit",
    label: "edit",
  },
];
