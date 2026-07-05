import { Separator } from "@/components/ui/separator";
import DriverSelect from "./DriverSelect";
import LayoutSelect from "./LayoutSelect";
import ReferenceSelect from "./ReferenceSelect";
import SampleRate from "./SampleRate";
import Display from "./Display";
import Compare from "./Compare";
import Record from "./Record";

export default function NormalLayout() {
  return (
    <>
      <DriverSelect />
      <Separator orientation="vertical" className="mx-2 my-1" />
      <ReferenceSelect />
      <Separator orientation="vertical" className="mx-2 my-1" />
      <LayoutSelect />
      <Separator orientation="vertical" className="mx-2 my-1" />
      <SampleRate />
      <Separator orientation="vertical" className="mx-2 my-1" />
      <Display />
      <Separator orientation="vertical" className="mx-2 my-1" />
      <Record />
      <Separator orientation="vertical" className="mx-2 my-1" />
      <Compare />
    </>
  );
}
