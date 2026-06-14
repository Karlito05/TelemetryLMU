import { MenuProps, Dropdown, Button } from "antd";
import { DownOutlined } from "@ant-design/icons";

type DriverLayoutWidgetProps = {
  drivers: MenuProps["items"];
  layouts: MenuProps["items"];
  onDriverSelect?: (key: string, driverName: string) => void;
  curDriver: string;
  onLayoutSelect?: (key: string) => void;
};

export default function DriverLayoutWidget({
  drivers,
  layouts,
  onDriverSelect,
  curDriver,
  onLayoutSelect,
}: DriverLayoutWidgetProps) {
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

//TODO: Fix this mess
