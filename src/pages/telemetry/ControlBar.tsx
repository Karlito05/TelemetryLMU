import { Dropdown, Button, MenuProps } from "antd";
import { DownOutlined } from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";
import { useContext, useEffect, useState } from "react";
import { GraphViewType } from "./Graphs";
import { TelemetryContext } from "./telemetry";

type Driver = {
  index: number;
  name: string;
};

export default function ControlBar() {
  const c = useContext(TelemetryContext);
  return (
    <div className="bg-[#FFFFFF18] h-full w-full rounded-3xl items-center flex justify-baseline p-2">
      {c.editMode ? <EditMode /> : <NormalMode />}
    </div>
  );
}

function EditMode() {
  const c = useContext(TelemetryContext);
  function handleAddGraph() {
    const nGD = [
      ...c.graphData,
      {
        baseColor: "#ff5d5d",
        carNum: c.curDriverNum,
        graphName: "Brake",
        nLines: 3,
        type: GraphViewType.Brake,
      },
    ];
    c.setGraphData(nGD);
    const newSizes: number[] = Array(nGD.length).fill(1 / nGD.length);
    c.setSizes(newSizes);
  }
  return (
    <>
      <Button onClick={() => c.setEditMode(false)}>Quit Edit Mode</Button>
      <Button onClick={() => handleAddGraph()}>Add Graph</Button>
    </>
  );
}

function NormalMode() {
  const [drivers, setDrivers] = useState<MenuProps["items"]>([]);
  const [curDriver, setCurDriver] = useState<string>("");
  const c = useContext(TelemetryContext);

  useEffect(() => {
    invoke<Driver[]>("get_drivers").then((v) => {
      setCurDriver(v[0].name);
      const items: MenuProps["items"] = v.map((driver) => ({
        label: driver.name,
        key: String(driver.index),
      }));

      setDrivers(items);
    });
  }, []);
  return (
    <>
      <DriverSelect
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

// TODO: Factor this out somehow

const layouts: MenuProps["items"] = [
  { type: "divider" },
  {
    key: "edit",
    label: "edit",
  },
];

type DriverSelectProps = {
  drivers: MenuProps["items"];
  layouts: MenuProps["items"];
  onDriverSelect?: (key: string, driverName: string) => void;
  curDriver: string;
  onLayoutSelect?: (key: string) => void;
};

function DriverSelect({
  drivers,
  layouts,
  onDriverSelect,
  curDriver,
  onLayoutSelect,
}: DriverSelectProps) {
  return (
    <div className="flex flex-col m-2 space-y-1">
      <div className="text-white flex items-center justify-between">
        Driver:
        <Dropdown
          trigger={["click"]}
          menu={{
            items: drivers,
            onClick: ({ key }) => {
              const selectedDriver = drivers?.find(
                (driver) =>
                  driver &&
                  typeof driver === "object" &&
                  "key" in driver &&
                  driver.key === key,
              );

              if (
                selectedDriver &&
                typeof selectedDriver === "object" &&
                "label" in selectedDriver &&
                typeof selectedDriver.label === "string"
              ) {
                onDriverSelect?.(key, selectedDriver.label);
              }
            },
          }}
        >
          <Button type="primary">
            {curDriver}
            <DownOutlined />
          </Button>
        </Dropdown>
      </div>
      <div className="text-white flex items-center justify-between">
        Layout:
        <Dropdown
          trigger={["click"]}
          menu={{ items: layouts, onClick: ({ key }) => onLayoutSelect?.(key) }}
        >
          <Button type="primary">
            Layout 1 <DownOutlined />
          </Button>
        </Dropdown>
      </div>
    </div>
  );
}
