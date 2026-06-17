import { useContext } from "react";
import { TelemetryContext } from "../Telemetry";
import { GraphViewType } from "./graphTypes";
import { ColorPicker, Button, Dropdown, InputNumber, MenuProps } from "antd";
import { DownOutlined } from "@ant-design/icons";

type GraphViewDummyProps = {
  index: number;
};

export default function GraphViewDummy({ index }: GraphViewDummyProps) {
  const c = useContext(TelemetryContext);
  function handleColorChange(index: number, color: string) {
    const newGD = [...c.graphData];
    newGD[index] = { ...newGD[index], baseColor: color };
    c.setGraphData(newGD);
  }

  function handleNLinesChange(index: number, nLines: number | null) {
    if (nLines === null || nLines === undefined) return;
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
    const newSizes: number[] = Array(nGD.length).fill(1 / nGD.length);
    c.setSizes(newSizes);
  }

  return (
    <div
      className={`w-full h-full`}
      style={{ backgroundColor: `${c.graphData[index].baseColor}40` }}
    >
      <ColorPicker
        value={c.graphData[index].baseColor}
        onChangeComplete={(color) =>
          handleColorChange(index, color.toHexString())
        }
        disabledAlpha={true}
      />
      <InputNumber
        min={3}
        max={10}
        value={c.graphData[index].nLines}
        onChange={(e) => handleNLinesChange(index, e)}
      />
      <Dropdown
        menu={{
          items: typeOptions,
          onClick: ({ key }) => handleTypeChange(index, key as GraphViewType),
        }}
      >
        <Button type="primary">
          {c.graphData[index].type.charAt(0).toUpperCase() +
            c.graphData[index].type.slice(1)}
          <DownOutlined />
        </Button>
      </Dropdown>
      <Button onClick={() => handleDelete(index)}>Delete</Button>
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
