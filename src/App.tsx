import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Kanban } from "../src-tauri/bindings/Kanban";
import { ReactSortable } from "react-sortablejs";
import { Task } from "../src-tauri/bindings/Task";
import { ulid } from "ulid";
import { open } from "@tauri-apps/api/dialog";
import Card from "@mui/material/Card";
import CardContent from "@mui/material/CardContent";
import AddIcon from "@mui/icons-material/Add";
import Modal from "@mui/material/Modal";
import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";

function App() {
  const [text, setText] = useState<string>("");
  const [todo, setTodo] = useState<Task[]>([]);
  const [inProgress, setInProgress] = useState<Task[]>([]);
  const [done, setDone] = useState<Task[]>([]);
  const [isOpen, setIsOpen] = useState<boolean>(false);
  const [isTaskModalOpen, setIsTaskModalOpen] = useState<boolean>(false);

  const handleCreateTask = async () => {
    if (text === "") return;

    const newTodo = {
      id: ulid(),
      name: text,
      is_complete: false,
    };
    await invoke("create_task_command", {
      task: newTodo,
    })
      .then(() => {
        setTodo([...todo, newTodo]);
        setText("");
      })
      .catch(() => {});
  };

  const handleMoveTask = async (from: string, to: string, oldIndex: number) => {
    let oldTask: Task | undefined;
    if (from === "todo") oldTask = todo[oldIndex];
    if (from === "in_progress") oldTask = inProgress[oldIndex];
    if (from === "done") oldTask = done[oldIndex];

    if (!oldTask) return;

    await invoke("move_task_command", {
      task: oldTask,
      from,
      to,
    });
  };

  const handleInitialSetting = async () => {
    await open({ multiple: false })
      .then(async (files) => {
        if (files === null) return;

        setIsOpen(false);
        await invoke<Kanban>("initial_setting_command", {
          path: files,
        })
          .then((data) => {
            setTodo(data.todo);
            setInProgress(data.in_progress);
            setDone(data.done);
          })
          .catch(() => {});
      })
      .catch(() => {});
  };

  const handleDeleteTask = async (e: any, id: string) => {
    const target = e.currentTarget.getAttribute("data-target");
    await invoke<Kanban>("delete_task_command", {
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

  const handleCreateTaskModal = () => {
    setIsTaskModalOpen(true);
  };

  useEffect(() => {
    invoke("check_path_command")
      .then((data) => {
        console.log("data", data);
        if (data) {
          invoke<Kanban>("get_task_command").then((todo) => {
            setTodo(todo.todo);
            setInProgress(todo.in_progress);
            setDone(todo.done);
          });
        } else {
          setIsOpen(true);
        }
      })
      .catch(() => {});
  }, []);

  return (
    <div className="wrapper">
      <div className="taskInput">
        <TextField
          onChange={(e) => setText(e.target.value)}
          value={text}
          label="タスク名を入力"
          size="small"
        />
        <Button variant="contained" onClick={handleCreateTask}>
          作成
        </Button>
      </div>
      <div className="container">
        <div className="column">
          <div className="columnHeader">
            <p className="columnName">Todo</p>
            <AddIcon onClick={handleCreateTaskModal} />
          </div>
          <ReactSortable
            group="groupName"
            animation={200}
            list={todo}
            setList={setTodo}
            onEnd={(e) => {
              handleMoveTask(e.from.id, e.to.id, e.oldIndex);
            }}
            className="columnTasks"
            id="todo"
            forceFallback={true}
          >
            {todo.map((item) => {
              return (
                <Card className="taskItem" key={item.id}>
                  <CardContent>
                    {item.name}
                    <span
                      className="deleteBtn"
                      data-target="todo"
                      onClick={(e) => handleDeleteTask(e, item.id)}
                    >
                      x
                    </span>
                  </CardContent>
                </Card>
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
              handleMoveTask(e.from.id, e.to.id, e.oldIndex);
            }}
            className="columnTasks"
            id="in_progress"
            forceFallback={true}
          >
            {inProgress.map((item) => {
              return (
                <Card className="taskItem" key={item.id}>
                  <CardContent>
                    {item.name}
                    <span
                      className="deleteBtn"
                      data-target="in_progress"
                      onClick={(e) => handleDeleteTask(e, item.id)}
                    >
                      x
                    </span>
                  </CardContent>
                </Card>
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
              handleMoveTask(e.from.id, e.to.id, e.oldIndex);
            }}
            className="columnTasks"
            id="done"
            forceFallback={true}
          >
            {done.map((item) => {
              return (
                <Card className="taskItem" key={item.id}>
                  <CardContent>
                    {item.name}
                    <span
                      className="deleteBtn"
                      data-target="done"
                      onClick={(e) => handleDeleteTask(e, item.id)}
                    >
                      x
                    </span>
                  </CardContent>
                </Card>
              );
            })}
          </ReactSortable>
        </div>
      </div>
      {isOpen && (
        <Modal open={isOpen}>
          <Card className="settingModal">
            <p>タスクファイルを選択してください</p>
            <div>
              <button onClick={handleInitialSetting}>ファイル</button>
            </div>
          </Card>
        </Modal>
      )}
      <Modal open={isTaskModalOpen} onClose={() => setIsTaskModalOpen(false)}>
        <Card>
          <p>hello</p>
        </Card>
      </Modal>
    </div>
  );
}

export default App;
