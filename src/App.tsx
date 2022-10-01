import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Memo } from "../src-tauri/bindings/Memo";
import { ReactSortable } from "react-sortablejs";
import { Todo } from "../src-tauri/bindings/Todo";
import { ulid } from "ulid";
import { open } from "@tauri-apps/api/dialog";

function App() {
  const [text, setText] = useState<string>("");
  const [todo, setTodo] = useState<Todo[]>([]);
  const [inProgress, setInProgress] = useState<Todo[]>([]);
  const [done, setDone] = useState<Todo[]>([]);
  const [isOpen, setIsOpen] = useState<boolean>(true);
  const [taskPath, setTaskPath] = useState<string | string[]>("");

  async function handleCreateMemo() {
    const newTodo = {
      id: ulid(),
      name: text,
      is_complete: false,
    };
    await invoke("create_memo_command", {
      taskPath,
      argMemo: newTodo,
    })
      .then((result) => {
        console.log("result", result);
        setTodo([...todo, newTodo]);
      })
      .catch(() => {});
  }

  async function handleMoveMemo(from: string, to: string, oldIndex: number) {
    let oldData: Todo | undefined;
    if (from === "todo") oldData = todo[oldIndex];
    if (from === "in_progress") oldData = inProgress[oldIndex];
    if (from === "done") oldData = done[oldIndex];

    if (!oldData) return;

    await invoke("move_memo_command", {
      taskPath,
      argMemo: oldData,
      from,
      to,
    });
  }

  const handleInitialSetting = async () => {
    await open({ multiple: false })
      .then(async (files) => {
        if (files === null) return;

        setIsOpen(false);
        await invoke<Memo>("initial_setting_command", {
          path: files,
        })
          .then((data) => {
            setTaskPath(files);
            setTodo(data.todo);
            setInProgress(data.in_progress);
            setDone(data.done);
          })
          .catch(() => {});
      })
      .catch(() => {});
  };

  return (
    <>
      <div>
        <input type="text" onChange={(e) => setText(e.target.value)} />
        <button onClick={handleCreateMemo}>作成</button>
      </div>
      <div className="container">
        <ReactSortable
          group="groupName"
          animation={200}
          list={todo}
          setList={setTodo}
          onEnd={(e) => {
            console.log("e", e);
            handleMoveMemo(e.from.id, e.to.id, e.oldIndex);
          }}
          className="column"
          id="todo"
        >
          {todo.map((item) => {
            return (
              <div className="taskItem" key={item.id}>
                {item.name}
              </div>
            );
          })}
        </ReactSortable>
        <ReactSortable
          group="groupName"
          animation={200}
          list={inProgress}
          setList={setInProgress}
          onEnd={(e) => {
            console.log("e", e);
            handleMoveMemo(e.from.id, e.to.id, e.oldIndex);
          }}
          className="column"
          id="in_progress"
        >
          {inProgress.map((item) => {
            return (
              <div className="taskItem" key={item.id}>
                {item.name}
              </div>
            );
          })}
        </ReactSortable>
        <ReactSortable
          group="groupName"
          animation={200}
          list={done}
          setList={setDone}
          onEnd={(e) => {
            console.log("e", e);
            handleMoveMemo(e.from.id, e.to.id, e.oldIndex);
          }}
          className="column"
          id="done"
        >
          {done.map((item) => {
            return (
              <div className="taskItem" key={item.id}>
                {item.name}
              </div>
            );
          })}
        </ReactSortable>
      </div>
      {isOpen && (
        <div className="cover">
          <div className="settingModal">
            <p>タスクファイルを選択してください</p>
            <div>
              <button onClick={handleInitialSetting}>ファイル</button>
            </div>
          </div>
        </div>
      )}
    </>
  );
}

export default App;
