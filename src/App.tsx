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

  const handleCreateMemo = async () => {
    if (text === "") return;

    const newTodo = {
      id: ulid(),
      name: text,
      is_complete: false,
    };
    await invoke("create_memo_command", {
      taskPath,
      argMemo: newTodo,
    })
      .then(() => {
        setTodo([...todo, newTodo]);
        setText("");
      })
      .catch(() => {});
  };

  const handleMoveMemo = async (from: string, to: string, oldIndex: number) => {
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
  };

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

  const handleDeleteMemo = async (e: any, id: string) => {
    if (taskPath === null) return;
    const target = e.currentTarget.getAttribute("data-target");
    await invoke<Memo>("delete_memo_command", {
      path: taskPath,
      target,
      id,
    })
      .then(() => {
        if (target === "todo") {
          const targetIndex = todo.findIndex((item) => item.id === id);
          todo.splice(targetIndex, 1);
          setTodo([...todo]);
          return;
        }
        if (target === "in_progress") {
          const targetIndex = inProgress.findIndex((item) => item.id === id);
          inProgress.splice(targetIndex, 1);
          setInProgress([...inProgress]);
          return;
        }
        if (target === "done") {
          const targetIndex = done.findIndex((item) => item.id === id);
          done.splice(targetIndex, 1);
          setDone([...done]);
          return;
        }
      })
      .catch(() => {});
  };

  return (
    <div className="wrapper">
      <div className="taskInput">
        <input
          type="text"
          onChange={(e) => setText(e.target.value)}
          value={text}
        />
        <button onClick={handleCreateMemo}>作成</button>
      </div>
      <div className="container">
        <div className="column">
          <p className="columnName">Todo</p>
          <ReactSortable
            group="groupName"
            animation={200}
            list={todo}
            setList={setTodo}
            onEnd={(e) => {
              handleMoveMemo(e.from.id, e.to.id, e.oldIndex);
            }}
            className="columnTasks"
            id="todo"
          >
            {todo.map((item) => {
              return (
                <div className="taskItem" key={item.id}>
                  {item.name}
                  <span
                    className="deleteBtn"
                    data-target="todo"
                    onClick={(e) => handleDeleteMemo(e, item.id)}
                  >
                    x
                  </span>
                </div>
              );
            })}
          </ReactSortable>
        </div>
        <div className="column">
          <p className="columnName">In Progress</p>
          <ReactSortable
            group="groupName"
            animation={200}
            list={inProgress}
            setList={setInProgress}
            onEnd={(e) => {
              handleMoveMemo(e.from.id, e.to.id, e.oldIndex);
            }}
            className="columnTasks"
            id="in_progress"
          >
            {inProgress.map((item) => {
              return (
                <div className="taskItem" key={item.id}>
                  {item.name}
                  <span
                    className="deleteBtn"
                    data-target="in_progress"
                    onClick={(e) => handleDeleteMemo(e, item.id)}
                  >
                    x
                  </span>
                </div>
              );
            })}
          </ReactSortable>
        </div>
        <div className="column">
          <p className="columnName">Done</p>
          <ReactSortable
            group="groupName"
            animation={200}
            list={done}
            setList={setDone}
            onEnd={(e) => {
              handleMoveMemo(e.from.id, e.to.id, e.oldIndex);
            }}
            className="columnTasks"
            id="done"
          >
            {done.map((item) => {
              return (
                <div className="taskItem" key={item.id}>
                  {item.name}
                  <span
                    className="deleteBtn"
                    data-target="done"
                    onClick={(e) => handleDeleteMemo(e, item.id)}
                  >
                    x
                  </span>
                </div>
              );
            })}
          </ReactSortable>
        </div>
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
    </div>
  );
}

export default App;
