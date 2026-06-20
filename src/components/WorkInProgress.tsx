import { Empty, EmptyContent, EmptyHeader } from "./ui/empty";

export default function WorkInProgress() {
  return (
    <Empty>
      <EmptyHeader>Work in progress</EmptyHeader>
      <EmptyContent>There's nothing here at the moment :(</EmptyContent>
    </Empty>
  );
}
