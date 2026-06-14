import GraphView from "./GraphView";
import { DownOutlined } from "@ant-design/icons";
import { Dropdown, Splitter } from "antd";
import { ColorPicker, Input, InputNumber, MenuProps, Button } from "antd";
import { useContext } from "react";
import { TelemetryContext } from "./telemetry";

export enum GraphViewType {
  Throttle = "throttle",
  Brake = "brake",
  Rpm = "rpm",
  Delta = "delta",
  Speed = "speed",
}

export type GraphViewData = {
  baseColor: string;
  // graphName: string;
  nLines: number;
  type: GraphViewType;
};

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

export default function Graphs() {
  const c = useContext(TelemetryContext);
  return c.editMode ? (
    <Splitter vertical={true} className="w-full h-full" onResize={c.setSizes}>
      {c.graphData.map((data, i) => (
        <Splitter.Panel
          key={data.type}
          resizable={c.editMode}
          size={c.sizes[i]}
        >
          <GraphViewDummy index={i} />
        </Splitter.Panel>
      ))}
    </Splitter>
  ) : (
    <Splitter vertical={true} className="w-full h-full" onResize={c.setSizes}>
      {c.graphData.map((data, i) => (
        <Splitter.Panel
          key={data.type}
          resizable={c.editMode}
          size={c.sizes[i]}
        >
          <GraphView
            {...data}
            graphName={data.type.charAt(0).toUpperCase() + data.type.slice(1)}
            carNum={c.curDriverNum}
          />
        </Splitter.Panel>
      ))}
    </Splitter>
  );
}

type GraphViewDummyProps = {
  index: number;
};

function GraphViewDummy({ index }: GraphViewDummyProps) {
  const c = useContext(TelemetryContext);
  function handleColorChange(index: number, color: string) {
    const newGD = [...c.graphData];
    newGD[index] = { ...newGD[index], baseColor: color };
    c.setGraphData(newGD);
  }

  // function handleNameChange(index: number, name: string) {
  //   const newGD = [...c.graphData];
  //   newGD[index] = { ...newGD[index], graphName: name };
  //   c.setGraphData(newGD);
  // }

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
      {/* <Input
        value={c.graphData[index].graphName}
        onChange={(e) => handleNameChange(index, e.target.value)}
      /> */}
      <InputNumber
        min={2}
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
