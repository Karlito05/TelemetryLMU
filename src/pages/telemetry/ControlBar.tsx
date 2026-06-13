import { Dropdown, Button, MenuProps } from "antd";
import { DownOutlined } from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

type Driver = {
  index: number;
  name: string;
};

export default function ControlBar() {
  const [drivers, setDrivers] = useState<MenuProps["items"]>([]);

  useEffect(() => {
    invoke<Driver[]>("get_drivers").then((v) => {
      let items: MenuProps["items"] = v.map((driver) => ({
        label: driver.name,
        key: String(driver.index),
      }));

      setDrivers(items);
    });
  }, []);

  return (
    <div className="bg-[#FFFFFF18] h-full w-full rounded-3xl items-center flex justify-baseline p-2">
      <DriverSelect drivers={drivers} layouts={layouts} />
      <Spacer />
    </div>
  );
}

function Spacer() {
  return <div className="h-90/100 w-0.5 bg-[#FFFFFF40] rounded-full"></div>;
}

const layouts: MenuProps["items"] = [
  {
    key: "1",
    label: "l1",
  },
  {
    key: "2",
    label: "l2",
  },
  { type: "divider" },
  {
    key: "3",
    label: "edit",
  },
];

type DriverSelectProps = {
  drivers: MenuProps["items"];
  layouts: MenuProps["items"];
};

function DriverSelect({ drivers, layouts }: DriverSelectProps) {
  let driver = "K. Lukes";
  return (
    <div className="flex flex-col m-2 space-y-1">
      <div className="text-white flex items-center justify-between">
        Driver:
        <Dropdown trigger={["click"]} menu={{ items: drivers }}>
          <Button type="primary">
            {driver}
            <DownOutlined />
          </Button>
        </Dropdown>
      </div>
      <div className="text-white flex items-center justify-between">
        Layout:
        <Dropdown trigger={["click"]} menu={{ items: layouts }}>
          <Button type="primary">
            Layout 1 <DownOutlined />
          </Button>
        </Dropdown>
      </div>
    </div>
  );
}
