export type TaskTypes = "todo" | "doing" | "done";
export const taskTypes: TaskTypes[] = ["todo", "doing", "done"];

export type TaskGroupType = {
  [k in TaskTypes]?: TaskDataType[];
};

export type TaskDataType = {
  id: string;
  name: string;
  group: TaskTypes;
};
