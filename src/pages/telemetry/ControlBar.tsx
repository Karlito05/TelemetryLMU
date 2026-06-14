import { Dropdown, Button, MenuProps } from "antd";
import { DownOutlined } from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

type Driver = {
  index: number;
  name: string;
};

type ControlBarProps = {
  setCurDriverNum: React.Dispatch<React.SetStateAction<number>>;
  editMode: boolean;
  setEditMode: (value: boolean) => void;
};

export default function ControlBar({
  setCurDriverNum,
  editMode,
  setEditMode,
}: ControlBarProps) {
  return (
    <div className="bg-[#FFFFFF18] h-full w-full rounded-3xl items-center flex justify-baseline p-2">
      {editMode ? (
        <EditMode setEditMode={setEditMode} />
      ) : (
        <NormalMode
          setCurDriverNum={setCurDriverNum}
          setEditMode={setEditMode}
        />
      )}
    </div>
  );
}

type EditModeProps = {
  setEditMode: (value: boolean) => void;
};

function EditMode({ setEditMode }: EditModeProps) {
  return <Button onClick={() => setEditMode(false)}>Quit Edit Mode</Button>;
}

type NormalModeProps = {
  setCurDriverNum: React.Dispatch<React.SetStateAction<number>>;
  setEditMode: (value: boolean) => void;
};

function NormalMode({ setCurDriverNum, setEditMode }: NormalModeProps) {
  const [drivers, setDrivers] = useState<MenuProps["items"]>([]);
  const [curDriver, setCurDriver] = useState<string>("");

  useEffect(() => {
    invoke<Driver[]>("get_drivers").then((v) => {
      setCurDriver(v[0].name);
      let items: MenuProps["items"] = v.map((driver) => ({
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
          setCurDriverNum(carNum);
        }}
        onLayoutSelect={(key) => {
          if (key == "edit") {
            setEditMode(true);
          }
        }}
        curDriver={curDriver}
      />
      <Spacer />
    </>
  );
}
function Spacer() {
  return <div className="h-90/100 w-0.5 bg-[#FFFFFF40] rounded-full"></div>;
}

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
