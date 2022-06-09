import { useEffect, useState } from "react";
import "../css/App.css";
import { solve, init } from "../wasm";
import init_wasm from "../wasm/hello_wasm_bg.wasm";
// import * as xxx from "./wasm/hello_wasm_bg.wasm";

const MINE = 10;
const BLOCK = 11;
type board = number[][];
type Pos = [number, number];

function App() {
  const [count, setCount] = useState(0);
  const [inited, setInited] = useState(false);
  const [loading, setLoading] = useState(false);
  const [input, setInput] = useState("");
  const [error, setError] = useState("");
  const [errorPos, setErrorPos] = useState<Pos>([-1, -1]);
  const [result, setResult] = useState<JSX.Element>(<></>);
  async function onClick() {
    let hint;
    setError("");
    const board = input_convert(input);
    console.log("board :>> ", board);

    try {
      hint = JSON.parse(solve(JSON.stringify(board)));
      console.log(hint);
      if (!hint?.result) {
        throw hint.output;
      }
      const colorBoardLs: ColorBoard[] = [];
      const posList = hint.output;
      posList.forEach((pos: any) => {
        board[pos.position[0]][pos.position[1]] = pos.value;
        colorBoardLs.push({
          pos: pos.position,
          className: "hint",
        });
      });
      setResult(show_board(board, colorBoardLs));
    } catch (e) {
      const err = String(e);
      setError(err);
      setErrorPos(hint.position);
      setResult(
        show_board(board, [{ pos: hint.position, className: "error" }])
      );
    } finally {
      setLoading(false);
    }
  }
  useEffect(() => {
    // was

    (async () => {
      // @ts-ignore
      const wasm = await init_wasm();
      init(wasm);
      setInited(true);
    })();
  }, []);

  return (
    <div className="App">
      <>
        {loading ? <p>Loading...</p> : <></>}
        {inited ? <></> : <p>初期化中...</p>}
        {error ? <p>{error}</p> : <></>}
        <p>
          B:ブロック、M:地雷(Mine)
          <button style={{ marginLeft: "5px" }} onClick={onClick}>
            実行
          </button>
        </p>
        <textarea
          cols={40}
          rows={10}
          style={{ fontSize: "20px" }}
          onChange={(x) => setInput(x.target.value)}
        ></textarea>
        <br />
        <span>結果</span>
        <table>{result}</table>
        <p>
          例 <br />
          <table>
            <tbody>
              <tr>
                <td>ブロック</td>
                <td>1</td>
              </tr>
              <tr>
                <td>1</td>
                <td>1</td>
              </tr>
            </tbody>
          </table>
          <span>
            の場合
            <br />
          </span>
          B,1 <br />
          1,1 <br />
        </p>
        <p>0は特殊処理で無視されます(例えば周りに地雷があっても許可されます)</p>
        <p>端ではない場所を総当たりする場合は、正しく判定できるよう0で上手く調整してください</p>
      </>
    </div>
  );
}

function input_convert(s: string): board {
  const lines = s.split("\n");
  const board: board = [];
  lines.forEach((x, i) => {
    const line = x.trim().split(",");
    const row: number[] = [];
    line.forEach((a) => {
      a = a.trim();
      if (a.match(/^(B|b)$/)) {
        row.push(BLOCK);
      } else if (a.match(/^(M|m)$/)) {
        row.push(MINE);
      } else {
        row.push(Number(a));
      }
    });
    board.push(row);
  });
  return board;
}

function convert_to_string(num: number): string {
  if (num === BLOCK) {
    return "B";
  } else if (num === MINE) {
    return "💣";
  } else {
    return String(num);
  }
}

type ColorBoard = {
  className: string;
  pos: Pos;
};

function show_board(board: board, color_board: ColorBoard[]): JSX.Element {
  return (
    // <table>
    <tbody>
      {board.map((x, i) => {
        return (
          <tr key={i}>
            {x.map((y, j) => {
              const className =
                color_board.find((x) => x.pos[0] === i && x.pos[1] === j)
                  ?.className || "";
              return (
                <td key={j} className={className}>
                  {convert_to_string(y)}
                </td>
              );
            })}
          </tr>
        );
      })}
    </tbody>
    // </table>
  );
}

// function solve(board)

export default App;
