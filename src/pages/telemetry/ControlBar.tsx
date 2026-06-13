import { Dropdown, Button, MenuProps } from "antd";
import { DownOutlined } from "@ant-design/icons";

export default function ControlBar() {
  return (
    <div className="bg-[#FFFFFF18] h-full w-full rounded-3xl items-center flex justify-baseline p-2">
      <DriverSelect />
      <Spacer />
    </div>
  );
}

function Spacer() {
  return <div className="h-90/100 w-0.5 bg-[#FFFFFF40] rounded-full"></div>;
}

const drivers: MenuProps["items"] = [
  {
    key: "1",
    label: "Max",
  },
  {
    key: "2",
    label: "Paul Riciardo",
  },
  {
    key: "3",
    label: "Karel",
  },
];

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

function DriverSelect() {
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
