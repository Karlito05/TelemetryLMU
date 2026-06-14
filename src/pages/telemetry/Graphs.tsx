import GraphView from "./GraphView";
import { DownOutlined } from "@ant-design/icons";
import { Dropdown, Splitter } from "antd";
import { useState } from "react";
import { ColorPicker, Input, InputNumber, MenuProps, Button } from "antd";

type GraphsProps = {
  curDriverNum: number;
  editMode: boolean;
};

enum GraphViewType {
  Throttle = "throttle",
  Brake = "brake",
  Rpm = "rpm",
  Delta = "delta",
  Speed = "speed",
}

type GraphViewData = {
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

export default function Graphs({ curDriverNum, editMode }: GraphsProps) {
  const [sizes, setSizes] = useState<(number | string)[]>(["25%", "75%"]);
  const [graphData, setGraphData] = useState<GraphViewData[]>([
    {
      baseColor: "#9eff5d",
      carNum: curDriverNum,
      graphName: "Throttle",
      nLines: 3,
      type: GraphViewType.Throttle,
    },
    {
      baseColor: "#ff5d5d",
      carNum: curDriverNum,
      graphName: "Brake",
      nLines: 3,
      type: GraphViewType.Brake,
    },
  ]);
  return editMode ? (
    <Splitter vertical={true} className="w-full h-full" onResize={setSizes}>
      {graphData.map((data, i) => (
        <Splitter.Panel key={data.type} resizable={editMode} size={sizes[i]}>
          <GraphViewDummy
            graphData={graphData}
            setGraphData={setGraphData}
            index={i}
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
};

function GraphViewDummy({
  graphData,
  setGraphData,
  index,
}: GraphViewDummyProps) {
  function handleColorChange(index: number, color: string) {
    graphData[index].baseColor = color;
    setGraphData(graphData);
  }

  function handleNameChange(index: number, name: string) {
    graphData[index].graphName = name;
    setGraphData(graphData);
  }

  function handleNLinesChange(index: number, nLines: number | null) {
    if (!nLines) return;
    graphData[index].nLines = nLines;
    setGraphData(graphData);
  }

  function handleTypeChange(index: number, type: GraphViewType) {
    graphData[index].type = type;
    setGraphData(graphData);
  }

  return (
    <div className="w-full h-full bg-[#FFFFFF40]">
      <ColorPicker
        defaultValue={graphData[index].baseColor}
        onChangeComplete={(color) =>
          handleColorChange(index, color.toCssString())
        }
      />
      <Input
        defaultValue={graphData[index].graphName}
        onChange={(e) => handleNameChange(index, e.target.value)}
      />
      <InputNumber
        min={2}
        max={10}
        defaultValue={3}
        onChange={(e) => handleNLinesChange(index, e)}
      />
      <Dropdown
        menu={{
          items: typeOptions,
          onClick: ({ key }) => handleTypeChange(index, key as GraphViewType),
        }}
      >
        <Button type="primary">
          {graphData[index].type} <DownOutlined />
        </Button>
      </Dropdown>
    </div>
  );
}
