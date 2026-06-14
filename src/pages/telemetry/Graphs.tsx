import GraphView from "./GraphView";
import { DownOutlined } from "@ant-design/icons";
import { Dropdown, Splitter } from "antd";
import { useState } from "react";
import { ColorPicker, Input, InputNumber, MenuProps, Button } from "antd";

type GraphsProps = {
  editMode: boolean;
  graphData: GraphViewData[];
  setGraphData: (value: GraphViewData[]) => void;
  sizes: number[];
  setSizes: (value: number[]) => void;
};

export enum GraphViewType {
  Throttle = "throttle",
  Brake = "brake",
  Rpm = "rpm",
  Delta = "delta",
  Speed = "speed",
}

export type GraphViewData = {
  baseColor: string;
  carNum: number;
  graphName: string;
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

export default function Graphs({
  editMode,
  graphData,
  setGraphData,
  sizes,
  setSizes,
}: GraphsProps) {
  return editMode ? (
    <Splitter vertical={true} className="w-full h-full" onResize={setSizes}>
      {graphData.map((data, i) => (
        <Splitter.Panel key={data.type} resizable={editMode} size={sizes[i]}>
          <GraphViewDummy
            graphData={graphData}
            setGraphData={setGraphData}
            index={i}
            sizes={sizes}
            setSizes={setSizes}
          />
        </Splitter.Panel>
      ))}
    </Splitter>
  ) : (
    <Splitter vertical={true} className="w-full h-full" onResize={setSizes}>
      {graphData.map((data, i) => (
        <Splitter.Panel key={data.type} resizable={editMode} size={sizes[i]}>
          <GraphView {...data} />
        </Splitter.Panel>
      ))}
    </Splitter>
  );
}

type GraphViewDummyProps = {
  graphData: GraphViewData[];
  setGraphData: (graphData: GraphViewData[]) => void;
  index: number;
  sizes: number[];
  setSizes: (value: number[]) => void;
};

function GraphViewDummy({
  graphData,
  setGraphData,
  index,
  sizes,
  setSizes,
}: GraphViewDummyProps) {
  function handleColorChange(index: number, color: string) {
    const newGD = [...graphData];
    newGD[index] = { ...newGD[index], baseColor: color };
    setGraphData(newGD);
  }

  function handleNameChange(index: number, name: string) {
    const newGD = [...graphData];
    newGD[index] = { ...newGD[index], graphName: name };
    setGraphData(newGD);
  }

  function handleNLinesChange(index: number, nLines: number | null) {
    if (nLines === null || nLines === undefined) return;
    const newGD = [...graphData];
    newGD[index] = { ...newGD[index], nLines };
    setGraphData(newGD);
  }

  function handleTypeChange(index: number, type: GraphViewType) {
    const newGD = [...graphData];
    newGD[index] = { ...newGD[index], type };
    setGraphData(newGD);
  }

  function handleDelete(index: number) {
    const nGD = [...graphData.slice(0, index), ...graphData.slice(index + 1)];
    setGraphData(nGD);
    const newSizes: number[] = Array(nGD.length).fill(1 / nGD.length);
    setSizes(newSizes);
  }

  return (
    <div
      className={`w-full h-full`}
      style={{ backgroundColor: `${graphData[index].baseColor}40` }}
    >
      <ColorPicker
        value={graphData[index].baseColor}
        onChangeComplete={(color) =>
          handleColorChange(index, color.toHexString())
        }
        disabledAlpha={true}
      />
      <Input
        value={graphData[index].graphName}
        onChange={(e) => handleNameChange(index, e.target.value)}
      />
      <InputNumber
        min={2}
        max={10}
        value={graphData[index].nLines}
        onChange={(e) => handleNLinesChange(index, e)}
      />
      <Dropdown
        menu={{
          items: typeOptions,
          onClick: ({ key }) => handleTypeChange(index, key as GraphViewType),
        }}
      >
        <Button type="primary">
          {/*Fix: doesn't update in the frontend somehow*/}
          {graphData[index].type} <DownOutlined />
        </Button>
      </Dropdown>
      <Button onClick={() => handleDelete(index)}>Delete</Button>
    </div>
  );
}
