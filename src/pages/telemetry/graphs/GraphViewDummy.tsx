import { useContext } from "react";
import { TelemetryContext } from "../Telemetry";
import { Dropdown, MenuProps } from "antd";
import { GraphViewType } from "../store";
import { DownOutlined } from "@ant-design/icons";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { NumberInput } from "@/components/ui/number-input";
type GraphViewDummyProps = {
  index: number;
  style?: React.CSSProperties;
};

export default function GraphViewDummy({ index, style }: GraphViewDummyProps) {
  const c = useContext(TelemetryContext);
  function handleColorChange(index: number, color: string) {
    const newGD = [...c.graphData];
    newGD[index] = { ...newGD[index], baseColor: color };
    c.setGraphData(newGD);
  }

  function handleNLinesChange(index: number, nLines: number | null) {
    if (nLines === null || nLines === undefined) return;
    if (nLines < 3 || nLines > 10) return;
    const newGD = [...c.graphData];
    newGD[index] = { ...newGD[index], nLines };
    c.setGraphData(newGD);
  }

  function handleTypeChange(index: number, type: GraphViewType) {
    const newGD = [...c.graphData];
    newGD[index] = { ...newGD[index], type };
    c.setGraphData(newGD);
  }

  function handleDelete(index: number) {
    const nGD = [
      ...c.graphData.slice(0, index),
      ...c.graphData.slice(index + 1),
    ];
    c.setGraphData(nGD);
    const newSizes: string[] = Array(nGD.length).fill(
      (1 / nGD.length).toString() + "%",
    );
    c.setSizes(newSizes);
  }

  return (
    <div
      className={
        "w-full h-full flex gap-6 text-white text-2xl font-[Electrolize] p-3"
      }
      style={{ backgroundColor: `${c.graphData[index].baseColor}40`, ...style }}
    >
      {/* <div className="flex gap-3 h-fit">
        {"Color:"}
        <ColorPicker
          showText
          value={c.graphData[index].baseColor}
          onChangeComplete={(color) =>
            handleColorChange(index, color.toHexString())
          }
          disabledAlpha={true}
        />
      </div> */}
      <div className="flex gap-3 h-fit">
        {"Gridlines:"}
        <NumberInput
          defaultValue={c.graphData[index].nLines ?? ""}
          onValueChange={(v) => handleNLinesChange(index, v)}
          min={3}
          max={10}
        />
      </div>

      <div className="flex gap-3 h-fit">
        {"Graph Type:"}
        <Dropdown
          menu={{
            items: typeOptions,
            onClick: ({ key }) => handleTypeChange(index, key as GraphViewType),
          }}
        >
          <Button>
            {c.graphData[index].type.charAt(0).toUpperCase() +
              c.graphData[index].type.slice(1)}
            <DownOutlined />
          </Button>
        </Dropdown>
      </div>

      <div className="flex gap-3 h-fit">
        <Button
          onClick={() => handleDelete(index)}
          style={{ background: "#C00000" }}
        >
          Delete
        </Button>
      </div>
    </div>
  );
}

//TODO: Somehow link this to one main type
const typeOptions: MenuProps["items"] = [
  {
    key: GraphViewType.Throttle,
    label: "Throttle",
  },
  {
    key: GraphViewType.Brake,
    label: "Brake",
  },
  {
    key: GraphViewType.Rpm,
    label: "Rpm",
  },
  {
    key: GraphViewType.Delta,
    label: "Delta",
  },
  {
    key: GraphViewType.Speed,
    label: "Speed",
  },
];
