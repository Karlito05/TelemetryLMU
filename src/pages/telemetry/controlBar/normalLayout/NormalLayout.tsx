import { useEffect, useState, useContext } from "react";
import { invoke } from "@tauri-apps/api/core";
import { TelemetryContext } from "../../Telemetry";
import { MenuProps } from "antd";
import DriverLayoutWidget from "./DriverLayoutsWidget";

export default function NormalLayout() {
  const [drivers, setDrivers] = useState<MenuProps["items"]>([]);
  const [curDriver, setCurDriver] = useState("");
  const [curLayout, setCurLayout] = useState("");
  const c = useContext(TelemetryContext);
  const layouts: MenuProps["items"] = [
    ...c.layouts.map((data, i) => {
      return { label: data.name, key: i.toString() };
    }),
    { type: "divider" },
    {
      key: "edit",
      label: "edit",
    },
  ];
  function handleLayout(key: string) {
    if (key == "edit") {
      c.setEditMode(true);
    } else {
      c.setActiveLayout(Number(key));
      setCurLayout(c.layouts[Number(key)].name);
    }
  }

  useEffect(() => {
    invoke<Driver[]>("get_drivers").then((v) => {
      try {
        setCurDriver(v[0].name);
      } catch (e) {
        console.error("No drivers found. The game is probably not runing.");
        setCurDriver("N/A");
      }
      setCurLayout(c.layouts[0].name);
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
        onLayoutSelect={handleLayout}
        curDriver={curDriver}
        curLayout={curLayout}
      />
    </>
  );
}

type Driver = {
  index: number;
  name: string;
};
