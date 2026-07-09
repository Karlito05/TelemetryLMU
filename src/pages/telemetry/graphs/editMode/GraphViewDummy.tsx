import { useContext } from "react";
import { TelemetryContext } from "../../Telemetry";
import { GraphViewType } from "../../store";
import { Button } from "@/components/ui/button";
import { NumberInput } from "@/components/ui/number-input";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { ColorPicker } from "@/components/ui/color-picker";

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
    const nGD = [...c.graphData.slice(0, index), ...c.graphData.slice(index + 1)];
    c.setGraphData(nGD);
    const newSizes: string[] = Array(nGD.length).fill((1 / nGD.length).toString() + "%");
    c.setSizes(newSizes);
  }

  return (
    <div
      className={
        "w-full h-full flex gap-6 text-white text-xl font-[Electrolize] p-3 bg-transparent z-50 "
      }
      style={{ ...style }}
    >
      <div className="flex gap-3 h-fit justify-center items-center">
        {"Color: "}
        <ColorPicker
          size={"sm"}
          value={c.graphData[index].baseColor}
          onChange={(v) => {
            handleColorChange(index, v.toString());
          }}
        />
      </div>
      <div className="flex gap-3 h-fit justify-center items-center">
        {"Gridlines:"}
        <div className="flex">
          <NumberInput
            defaultValue={c.graphData[index].nLines ?? ""}
            onValueChange={(v) => handleNLinesChange(index, v)}
            min={3}
            max={10}
          />
        </div>
      </div>
      <div className="flex gap-3 h-fit justify-center items-center">
        {"Graph Type:"}
        <Select
          value={c.graphData[index].type}
          onValueChange={(value) => handleTypeChange(index, value as GraphViewType)}
        >
          <SelectTrigger>
            <SelectValue placeholder="Select graph type" />
          </SelectTrigger>
          <SelectContent position="popper">
            <SelectGroup>
              {Object.values(GraphViewType).map((type) => (
                <SelectItem key={type} value={type}>
                  {type.charAt(0).toUpperCase() + type.slice(1)}
                </SelectItem>
              ))}
            </SelectGroup>
          </SelectContent>
        </Select>
      </div>
      <div className="flex gap-3 h-fit">
        <Button onClick={() => handleDelete(index)} className="bg-destructive">
          Delete
        </Button>
      </div>
    </div>
  );
}
